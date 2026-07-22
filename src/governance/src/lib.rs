/*!
 * Governance Module - Manages DAO voting, licensing, and global standards
 */

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::{generate_id, LicenseId, MemberId, Region};

/// License for operating a Studio Commons hub
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub id: LicenseId,
    pub region: Region,
    pub duration_months: u32,
    pub licensee: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub auction_price: u128,
    pub standards_compliant: bool,
}

/// Governance proposal types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    NewHub,
    PolicyChange,
    BudgetAllocation,
    StandardsUpdate,
    Other(String),
}

/// Proposal for DAO voting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub proposer: MemberId,
    pub created_at: DateTime<Utc>,
    pub voting_deadline: DateTime<Utc>,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Expired,
}

/// Vote on a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub proposal_id: uuid::Uuid,
    pub voter: MemberId,
    pub in_favor: bool,
    pub weight: f64, // Can be weighted by CCI score
    pub timestamp: DateTime<Utc>,
}

/// Vote result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResult {
    pub proposal_id: uuid::Uuid,
    pub total_votes: usize,
    pub votes_in_favor: usize,
    pub votes_against: usize,
    pub passed: bool,
}

/// Global standards metrics — universal, identity-blind measures that mean
/// the same thing in every culture: open access, fair pay, sustainable
/// operations, and members who are actually satisfied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMetrics {
    /// Percentage of membership applications accepted purely on published,
    /// objective criteria (dues paid, code of conduct). 100 = fully open.
    pub open_access_score: f64,
    /// Percentage of paid roles meeting or exceeding local fair-wage standards.
    pub fair_wage_compliance: f64,
    pub sustainability_score: f64,
    pub community_satisfaction: f64,
}

pub struct GovernanceSystem {
    region: Region,
    licenses: Vec<License>,
    proposals: Vec<Proposal>,
    votes: HashMap<uuid::Uuid, Vec<Vote>>,
}

impl GovernanceSystem {
    pub fn new(region: &str) -> Result<Self> {
        info!("Initializing Governance System for region: {}", region);

        Ok(Self {
            region: Region::from_str(region),
            licenses: Vec::new(),
            proposals: Vec::new(),
            votes: HashMap::new(),
        })
    }

    /// Conduct a global auction for a studio license
    pub fn global_auction_license(
        &mut self,
        region: Region,
        duration_months: u32,
        licensee: String,
        auction_price: u128,
    ) -> Result<LicenseId> {
        info!(
            "Creating license for region: {} (duration: {} months, price: {})",
            region, duration_months, auction_price
        );

        let start_date = Utc::now();
        let end_date = start_date + Duration::days((duration_months * 30) as i64);

        let license = License {
            id: generate_id(),
            region: region.clone(),
            duration_months,
            licensee,
            start_date,
            end_date,
            auction_price,
            standards_compliant: false, // Verified separately
        };

        let id = license.id;
        self.licenses.push(license);

        info!("License {} created for {}", id, region);

        Ok(id)
    }

    /// Multi-signature voting on proposals
    pub fn multi_vote(&mut self, proposal: Proposal, voters: Vec<MemberId>) -> Result<VoteResult> {
        info!("Processing multi-vote for proposal: {}", proposal.title);

        let proposal_id = proposal.id;
        self.proposals.push(proposal);

        // Initialize vote tracking for this proposal
        self.votes.insert(proposal_id, Vec::new());

        // Auto-approve for demo (in production, this would wait for actual votes)
        let result = VoteResult {
            proposal_id,
            total_votes: voters.len(),
            votes_in_favor: voters.len(),
            votes_against: 0,
            passed: true,
        };

        info!("Vote result: {} in favor, {} against", result.votes_in_favor, result.votes_against);

        Ok(result)
    }

    /// Cast a vote on a proposal
    pub fn cast_vote(
        &mut self,
        proposal_id: uuid::Uuid,
        voter: MemberId,
        in_favor: bool,
        weight: f64,
    ) -> Result<()> {
        let vote = Vote {
            proposal_id,
            voter,
            in_favor,
            weight,
            timestamp: Utc::now(),
        };

        self.votes
            .entry(proposal_id)
            .or_insert_with(Vec::new)
            .push(vote);

        info!("Vote cast by {} on proposal {}", voter, proposal_id);

        Ok(())
    }

    /// Enforce global standards: open access, fair wages, sustainability.
    /// No quotas on who members are — only guarantees on how the hub treats
    /// everyone who shows up.
    pub fn enforce_global_standards(
        &mut self,
        license_id: LicenseId,
        metrics: GlobalMetrics,
    ) -> Result<bool> {
        info!("Checking compliance for license: {}", license_id);

        let open_access_ok = metrics.open_access_score >= 95.0;
        let fair_wage_ok = metrics.fair_wage_compliance >= 100.0;
        let sustainability_ok = metrics.sustainability_score >= 70.0;

        let compliant = open_access_ok && fair_wage_ok && sustainability_ok;

        // Update license compliance status
        if let Some(license) = self.licenses.iter_mut().find(|l| l.id == license_id) {
            license.standards_compliant = compliant;
        }

        if compliant {
            info!("✓ License {} is compliant with global standards", license_id);
        } else {
            info!("✗ License {} failed compliance check", license_id);
        }

        Ok(compliant)
    }

    /// Adapt governance to regional requirements
    pub fn adapt_governance(&self, region: Region) -> GovernanceConfig {
        info!("Adapting governance for region: {}", region);

        // Different regions may have different quorum requirements, voting periods, etc.
        match region {
            Region::LA | Region::NYC | Region::Atlanta => GovernanceConfig {
                quorum_percentage: 51.0,
                voting_period_days: 7,
                requires_kyc: false,
            },
            Region::Berlin | Region::London => GovernanceConfig {
                quorum_percentage: 60.0,
                voting_period_days: 14,
                requires_kyc: true, // GDPR compliance
            },
            Region::Mumbai => GovernanceConfig {
                quorum_percentage: 50.0,
                voting_period_days: 10,
                requires_kyc: true,
            },
            _ => GovernanceConfig {
                quorum_percentage: 51.0,
                voting_period_days: 7,
                requires_kyc: false,
            },
        }
    }

    /// Get all active proposals
    pub fn get_active_proposals(&self) -> Vec<&Proposal> {
        self.proposals
            .iter()
            .filter(|p| p.status == ProposalStatus::Active)
            .collect()
    }

    /// Get license by ID
    pub fn get_license(&self, license_id: LicenseId) -> Option<&License> {
        self.licenses.iter().find(|l| l.id == license_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub quorum_percentage: f64,
    pub voting_period_days: u32,
    pub requires_kyc: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_license() {
        let mut gov = GovernanceSystem::new("LA").unwrap();
        let license_id = gov
            .global_auction_license(
                Region::LA,
                12,
                "Studio Collective LA".to_string(),
                1000000,
            )
            .unwrap();

        assert!(gov.get_license(license_id).is_some());
    }

    #[test]
    fn test_enforce_standards() {
        let mut gov = GovernanceSystem::new("LA").unwrap();
        let license_id = gov
            .global_auction_license(Region::LA, 12, "Test".to_string(), 1000)
            .unwrap();

        let metrics = GlobalMetrics {
            open_access_score: 100.0,
            fair_wage_compliance: 100.0,
            sustainability_score: 80.0,
            community_satisfaction: 90.0,
        };

        let compliant = gov.enforce_global_standards(license_id, metrics).unwrap();
        assert!(compliant);
    }

    #[test]
    fn test_voting() {
        let mut gov = GovernanceSystem::new("LA").unwrap();
        let proposal = Proposal {
            id: uuid::Uuid::new_v4(),
            title: "Test Proposal".to_string(),
            description: "Test".to_string(),
            proposal_type: ProposalType::PolicyChange,
            proposer: uuid::Uuid::new_v4(),
            created_at: Utc::now(),
            voting_deadline: Utc::now() + Duration::days(7),
            status: ProposalStatus::Active,
        };

        let voters = vec![uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];
        let result = gov.multi_vote(proposal, voters).unwrap();

        assert!(result.passed);
    }
}
