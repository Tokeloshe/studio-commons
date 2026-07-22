/*!
 * CCI Module - Creative Contribution Index
 *
 * A merit-based contribution ledger. A member's share of revenue is a pure
 * function of the work they verifiably did and how their peers independently
 * judged its quality. The system is:
 *
 * - Identity-blind: no attribute of *who* a member is ever enters the score.
 *   Only what they did, for how long, and how well.
 * - Peer-reviewed: impact is the MEDIAN of at least MIN_REVIEWS independent
 *   reviews. The median resists both outlier reviews and small collusion
 *   rings in a way an average cannot.
 * - Conflict-free: members who contributed to a project may not review
 *   contributions on that same project.
 * - Bounded: hours are capped per contribution period so nobody can
 *   out-claim the clock.
 * - Deterministic: the same ledger always produces the same scores, so any
 *   member can independently recompute and audit every distribution.
 * - Tamper-evident: contributions form a hash chain; rewriting history
 *   invalidates every subsequent entry.
 */

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::{MemberId, ProjectId};

/// Minimum number of independent peer reviews before a contribution scores.
pub const MIN_REVIEWS: usize = 3;

/// Maximum claimable hours per contribution entry (one calendar week at a
/// sustainable 12h/day). Longer work is logged as multiple entries, each
/// reviewed on its own merits.
pub const MAX_HOURS_PER_ENTRY: f64 = 84.0;

/// Review scores are bounded to [0, 1].
pub const MAX_IMPACT: f64 = 1.0;

/// Types of creative contributions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    Direction,
    Acting,
    Writing,
    Cinematography,
    Editing,
    Sound,
    Production,
    Design,
    Technical,
    Community,
    Other(String),
}

/// A single peer review of a contribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReview {
    pub reviewer: MemberId,
    /// Quality judgment in [0, 1].
    pub impact_score: f64,
    pub timestamp: DateTime<Utc>,
}

/// Individual contribution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    pub id: uuid::Uuid,
    pub member_id: MemberId,
    pub project_id: ProjectId,
    pub contribution_type: ContributionType,
    pub hours: f64,
    pub reviews: Vec<PeerReview>,
    pub timestamp: DateTime<Utc>,
    /// Hash of the previous ledger entry, chaining the ledger together.
    pub prev_hash: u64,
    /// Hash of this entry's content combined with prev_hash.
    pub entry_hash: u64,
}

/// Merit points for a member: hours worked × median peer-reviewed impact,
/// summed over all scored contributions. Nothing else.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCIPoints {
    pub member_id: MemberId,
    pub total_points: f64,
    pub scored_contributions: usize,
}

/// Residual share calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualShare {
    pub member_id: MemberId,
    pub project_id: ProjectId,
    pub share_percentage: f64,
    pub amount: u128,
}

/// Ledger integrity report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub entries: usize,
    pub chain_valid: bool,
    pub first_invalid_entry: Option<uuid::Uuid>,
}

pub struct CCISystem {
    ledger: Vec<Contribution>,
    /// Members who contributed to each project — excluded as reviewers there.
    project_contributors: HashMap<ProjectId, Vec<MemberId>>,
    /// Reviews received before their contribution is appended.
    pending_reviews: HashMap<uuid::Uuid, Vec<PeerReview>>,
}

/// FNV-1a over the serialized entry. Tamper-evidence for local audit; swap
/// for a cryptographic hash when the ledger is externally anchored.
fn hash_bytes(bytes: &[u8], seed: u64) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325 ^ seed;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = xs.len();
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    }
}

impl CCISystem {
    pub fn new() -> Result<Self> {
        info!("Initializing CCI System (merit-based, identity-blind)");

        Ok(Self {
            ledger: Vec::new(),
            project_contributors: HashMap::new(),
            pending_reviews: HashMap::new(),
        })
    }

    /// Append a contribution to the tamper-evident ledger.
    ///
    /// Rules enforced here, not by policy documents:
    /// - hours must be positive and within MAX_HOURS_PER_ENTRY
    /// - reviewers must be distinct, must not be the contributor, and must
    ///   not themselves be contributors on the same project
    /// - review scores must lie in [0, MAX_IMPACT]
    pub fn add_contribution(
        &mut self,
        member_id: MemberId,
        project_id: ProjectId,
        contribution_type: ContributionType,
        hours: f64,
        reviews: Vec<PeerReview>,
    ) -> Result<uuid::Uuid> {
        if !(hours > 0.0 && hours <= MAX_HOURS_PER_ENTRY) {
            bail!(
                "hours must be in (0, {}]: got {}",
                MAX_HOURS_PER_ENTRY,
                hours
            );
        }

        let existing = self
            .project_contributors
            .entry(project_id)
            .or_insert_with(Vec::new);

        let mut seen_reviewers = Vec::new();
        for review in &reviews {
            if review.reviewer == member_id {
                bail!("self-review is not permitted");
            }
            if existing.contains(&review.reviewer) {
                bail!(
                    "reviewer {} is a contributor on project {} (conflict of interest)",
                    review.reviewer,
                    project_id
                );
            }
            if seen_reviewers.contains(&review.reviewer) {
                bail!("duplicate review from {}", review.reviewer);
            }
            if !(0.0..=MAX_IMPACT).contains(&review.impact_score) {
                bail!("impact score must be in [0, {}]", MAX_IMPACT);
            }
            seen_reviewers.push(review.reviewer);
        }

        if !existing.contains(&member_id) {
            existing.push(member_id);
        }

        let id = uuid::Uuid::new_v4();
        let prev_hash = self.ledger.last().map(|c| c.entry_hash).unwrap_or(0);

        let mut contribution = Contribution {
            id,
            member_id,
            project_id,
            contribution_type,
            hours,
            reviews,
            timestamp: Utc::now(),
            prev_hash,
            entry_hash: 0,
        };
        contribution.entry_hash = Self::compute_entry_hash(&contribution);

        info!(
            "Ledger entry {} for member {} ({} hours, {} reviews)",
            id,
            member_id,
            hours,
            contribution.reviews.len()
        );

        self.ledger.push(contribution);
        Ok(id)
    }

    fn compute_entry_hash(c: &Contribution) -> u64 {
        let mut zeroed = c.clone();
        zeroed.entry_hash = 0;
        let bytes = serde_json::to_vec(&zeroed).unwrap_or_default();
        hash_bytes(&bytes, c.prev_hash)
    }

    /// Add a peer review to an existing, already-appended contribution.
    /// Same conflict rules apply. Reviews accumulate; the contribution only
    /// starts scoring once it has MIN_REVIEWS of them.
    pub fn add_review(&mut self, contribution_id: uuid::Uuid, review: PeerReview) -> Result<()> {
        let contribution = self
            .ledger
            .iter()
            .find(|c| c.id == contribution_id)
            .ok_or_else(|| anyhow::anyhow!("unknown contribution {}", contribution_id))?;

        if review.reviewer == contribution.member_id {
            bail!("self-review is not permitted");
        }
        if self
            .project_contributors
            .get(&contribution.project_id)
            .map(|m| m.contains(&review.reviewer))
            .unwrap_or(false)
        {
            bail!("reviewer is a contributor on this project (conflict of interest)");
        }
        if contribution
            .reviews
            .iter()
            .chain(
                self.pending_reviews
                    .get(&contribution_id)
                    .into_iter()
                    .flatten(),
            )
            .any(|r| r.reviewer == review.reviewer)
        {
            bail!("duplicate review from {}", review.reviewer);
        }
        if !(0.0..=MAX_IMPACT).contains(&review.impact_score) {
            bail!("impact score must be in [0, {}]", MAX_IMPACT);
        }

        // Reviews live outside the hashed entry so the chain stays valid;
        // they are folded in at scoring time.
        self.pending_reviews
            .entry(contribution_id)
            .or_insert_with(Vec::new)
            .push(review);

        Ok(())
    }

    fn effective_reviews<'a>(&'a self, c: &'a Contribution) -> Vec<&'a PeerReview> {
        c.reviews
            .iter()
            .chain(self.pending_reviews.get(&c.id).into_iter().flatten())
            .collect()
    }

    /// Merit score for one contribution: hours × median(peer impact).
    /// Returns None until the contribution has MIN_REVIEWS reviews.
    fn contribution_points(&self, c: &Contribution) -> Option<f64> {
        let reviews = self.effective_reviews(c);
        if reviews.len() < MIN_REVIEWS {
            return None;
        }
        let impact = median(reviews.iter().map(|r| r.impact_score).collect());
        Some(c.hours * impact)
    }

    /// Compute merit scores for every member, from the ledger alone.
    /// Deterministic: the same ledger always yields the same scores.
    pub fn compute_scores(&self) -> HashMap<MemberId, CCIPoints> {
        let mut scores: HashMap<MemberId, CCIPoints> = HashMap::new();

        for c in &self.ledger {
            if let Some(points) = self.contribution_points(c) {
                let entry = scores.entry(c.member_id).or_insert(CCIPoints {
                    member_id: c.member_id,
                    total_points: 0.0,
                    scored_contributions: 0,
                });
                entry.total_points += points;
                entry.scored_contributions += 1;
            }
        }

        scores
    }

    /// Distribute residuals for a project, proportional to merit points
    /// earned on that project. Runs identically today or in ten years:
    /// contributions are permanent, so residuals follow the work forever.
    pub fn global_residuals(
        &self,
        residuals: u128,
        project_id: ProjectId,
        year: u32,
    ) -> Result<Vec<ResidualShare>> {
        info!(
            "Calculating residuals for project {} (year {})",
            project_id, year
        );

        let mut project_points: HashMap<MemberId, f64> = HashMap::new();
        for c in self.ledger.iter().filter(|c| c.project_id == project_id) {
            if let Some(points) = self.contribution_points(c) {
                *project_points.entry(c.member_id).or_insert(0.0) += points;
            }
        }

        let total_points: f64 = project_points.values().sum();
        if total_points <= 0.0 {
            return Ok(Vec::new());
        }

        // Deterministic ordering so every recomputation is byte-identical.
        let mut ranked: Vec<(MemberId, f64)> = project_points.into_iter().collect();
        ranked.sort_by(|a, b| a.0.cmp(&b.0));

        // Clamp each payout to what remains undistributed so float rounding
        // can never mint money: the sum of shares is <= residuals by
        // construction, even at u128 extremes where f64 loses precision.
        let mut remaining = residuals;
        let mut shares = Vec::with_capacity(ranked.len());
        for (member_id, points) in ranked {
            let share_percentage = (points / total_points) * 100.0;
            let amount =
                ((residuals as f64 * points / total_points) as u128).min(remaining);
            remaining -= amount;
            shares.push(ResidualShare {
                member_id,
                project_id,
                share_percentage,
                amount,
            });
        }

        info!(
            "Distributed {} in residuals to {} members",
            residuals,
            shares.len()
        );

        Ok(shares)
    }

    /// Verify the hash chain. Any rewritten entry invalidates itself and
    /// every entry after it.
    pub fn audit(&self) -> AuditReport {
        let mut prev_hash = 0u64;
        for c in &self.ledger {
            if c.prev_hash != prev_hash || Self::compute_entry_hash(c) != c.entry_hash {
                return AuditReport {
                    entries: self.ledger.len(),
                    chain_valid: false,
                    first_invalid_entry: Some(c.id),
                };
            }
            prev_hash = c.entry_hash;
        }
        AuditReport {
            entries: self.ledger.len(),
            chain_valid: true,
            first_invalid_entry: None,
        }
    }

    /// Hash of the newest ledger entry. Publish or anchor this externally
    /// (chain state, signed minutes, a pinned post) — a hash chain alone
    /// cannot detect truncation of its own tail, but an anchored head can.
    pub fn head_hash(&self) -> u64 {
        self.ledger.last().map(|c| c.entry_hash).unwrap_or(0)
    }

    /// Number of ledger entries.
    pub fn ledger_len(&self) -> usize {
        self.ledger.len()
    }

    /// Get merit score for a member.
    pub fn get_cci_score(&self, member_id: MemberId) -> Option<CCIPoints> {
        self.compute_scores().remove(&member_id)
    }
}

impl Default for CCISystem {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reviews(scores: &[f64]) -> Vec<PeerReview> {
        scores
            .iter()
            .map(|s| PeerReview {
                reviewer: uuid::Uuid::new_v4(),
                impact_score: *s,
                timestamp: Utc::now(),
            })
            .collect()
    }

    #[test]
    fn test_merit_scoring_uses_median() {
        let mut cci = CCISystem::new().unwrap();
        let member = uuid::Uuid::new_v4();
        let project = uuid::Uuid::new_v4();

        // One inflated review (1.0) cannot drag the score up: median of
        // [0.8, 0.8, 1.0] is 0.8.
        cci.add_contribution(
            member,
            project,
            ContributionType::Direction,
            80.0,
            reviews(&[0.8, 1.0, 0.8]),
        )
        .unwrap();

        let scores = cci.compute_scores();
        let points = scores.get(&member).unwrap();
        assert!((points.total_points - 64.0).abs() < 1e-9);
    }

    #[test]
    fn test_unreviewed_work_does_not_score() {
        let mut cci = CCISystem::new().unwrap();
        let member = uuid::Uuid::new_v4();

        cci.add_contribution(
            member,
            uuid::Uuid::new_v4(),
            ContributionType::Editing,
            40.0,
            reviews(&[0.9]), // below MIN_REVIEWS
        )
        .unwrap();

        assert!(cci.compute_scores().get(&member).is_none());
    }

    #[test]
    fn test_hour_cap_enforced() {
        let mut cci = CCISystem::new().unwrap();
        let result = cci.add_contribution(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            ContributionType::Writing,
            10_000.0,
            reviews(&[0.9, 0.9, 0.9]),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_no_self_review() {
        let mut cci = CCISystem::new().unwrap();
        let member = uuid::Uuid::new_v4();
        let mut rs = reviews(&[0.9, 0.9]);
        rs.push(PeerReview {
            reviewer: member,
            impact_score: 1.0,
            timestamp: Utc::now(),
        });

        let result = cci.add_contribution(
            member,
            uuid::Uuid::new_v4(),
            ContributionType::Sound,
            10.0,
            rs,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_project_contributors_cannot_review_each_other() {
        let mut cci = CCISystem::new().unwrap();
        let alice = uuid::Uuid::new_v4();
        let bob = uuid::Uuid::new_v4();
        let project = uuid::Uuid::new_v4();

        cci.add_contribution(
            alice,
            project,
            ContributionType::Direction,
            50.0,
            reviews(&[0.8, 0.8, 0.8]),
        )
        .unwrap();

        // Alice already contributed to this project, so she cannot review
        // Bob's work on it.
        let conflicted = vec![PeerReview {
            reviewer: alice,
            impact_score: 1.0,
            timestamp: Utc::now(),
        }];
        let result =
            cci.add_contribution(bob, project, ContributionType::Sound, 20.0, conflicted);
        assert!(result.is_err());
    }

    #[test]
    fn test_residuals_proportional_to_merit() {
        let mut cci = CCISystem::new().unwrap();
        let sarah = uuid::Uuid::new_v4();
        let marcus = uuid::Uuid::new_v4();
        let project = uuid::Uuid::new_v4();

        // Sarah: 200h at median 0.9 = 180 points
        cci.add_contribution(
            sarah,
            project,
            ContributionType::Direction,
            80.0,
            reviews(&[0.9, 0.9, 0.9]),
        )
        .unwrap();
        cci.add_contribution(
            sarah,
            project,
            ContributionType::Direction,
            80.0,
            reviews(&[0.9, 0.9, 0.9]),
        )
        .unwrap();
        cci.add_contribution(
            sarah,
            project,
            ContributionType::Direction,
            40.0,
            reviews(&[0.9, 0.9, 0.9]),
        )
        .unwrap();
        // Marcus: 80h at median 0.9 = 72 points
        cci.add_contribution(
            marcus,
            project,
            ContributionType::Sound,
            80.0,
            reviews(&[0.9, 0.9, 0.9]),
        )
        .unwrap();

        let shares = cci.global_residuals(100_000, project, 1).unwrap();
        assert_eq!(shares.len(), 2);
        let total: u128 = shares.iter().map(|s| s.amount).sum();
        assert!(total <= 100_000 && total >= 99_998);

        let sarah_share = shares.iter().find(|s| s.member_id == sarah).unwrap();
        // 180 / 252 ≈ 71.4%
        assert!((sarah_share.share_percentage - 100.0 * 180.0 / 252.0).abs() < 1e-6);
    }

    #[test]
    fn test_ledger_tamper_detection() {
        let mut cci = CCISystem::new().unwrap();
        for _ in 0..3 {
            cci.add_contribution(
                uuid::Uuid::new_v4(),
                uuid::Uuid::new_v4(),
                ContributionType::Technical,
                10.0,
                reviews(&[0.5, 0.5, 0.5]),
            )
            .unwrap();
        }
        assert!(cci.audit().chain_valid);

        // Quietly inflate an early entry's hours.
        cci.ledger[0].hours = 84.0;
        let report = cci.audit();
        assert!(!report.chain_valid);
        assert_eq!(report.first_invalid_entry, Some(cci.ledger[0].id));
    }

    #[test]
    fn test_determinism() {
        let mut cci = CCISystem::new().unwrap();
        let member = uuid::Uuid::new_v4();
        let project = uuid::Uuid::new_v4();
        cci.add_contribution(
            member,
            project,
            ContributionType::Acting,
            50.0,
            reviews(&[0.9, 0.7, 0.8]),
        )
        .unwrap();

        let a = cci.global_residuals(100_000, project, 1).unwrap();
        let b = cci.global_residuals(100_000, project, 1).unwrap();
        assert_eq!(a.len(), b.len());
        assert_eq!(a[0].amount, b[0].amount);
    }
}
