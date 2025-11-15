/*!
 * CCI Module - Creative Contribution Index tracking and residuals
 */

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::{MemberId, ProjectId};

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

/// Individual contribution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    pub member_id: MemberId,
    pub project_id: ProjectId,
    pub contribution_type: ContributionType,
    pub hours: f64,
    pub impact_score: f64,
    pub timestamp: DateTime<Utc>,
}

/// Input for CCI calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCIInput {
    pub contributions: Vec<Contribution>,
    pub diversity_bonus: bool,
    pub sustainability_bonus: bool,
}

/// CCI points for a member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CCIPoints {
    pub member_id: MemberId,
    pub base_points: f64,
    pub diversity_multiplier: f64,
    pub sustainability_multiplier: f64,
    pub total_points: f64,
}

/// Residual share calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualShare {
    pub member_id: MemberId,
    pub project_id: ProjectId,
    pub share_percentage: f64,
    pub amount: u128,
}

/// Bias detection alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasAlert {
    pub timestamp: DateTime<Utc>,
    pub alert_type: String,
    pub description: String,
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
}

pub struct CCISystem {
    contributions: Vec<Contribution>,
    cci_scores: HashMap<MemberId, CCIPoints>,
}

impl CCISystem {
    pub fn new() -> Result<Self> {
        info!("Initializing CCI System");

        Ok(Self {
            contributions: Vec::new(),
            cci_scores: HashMap::new(),
        })
    }

    /// Calculate intelligent CCI with AI weighting for diversity/sustainability
    pub fn intelligent_cci(&mut self, input: CCIInput, ai_weight: bool) -> Result<CCIPoints> {
        info!("Calculating CCI for {} contributions", input.contributions.len());

        // Aggregate contributions by member
        let mut member_contributions: HashMap<MemberId, Vec<&Contribution>> = HashMap::new();

        for contribution in &input.contributions {
            member_contributions
                .entry(contribution.member_id)
                .or_insert_with(Vec::new)
                .push(contribution);
        }

        // Calculate points for each member
        for (member_id, contribs) in member_contributions {
            let base_points: f64 = contribs
                .iter()
                .map(|c| c.hours * c.impact_score)
                .sum();

            let diversity_multiplier = if input.diversity_bonus && ai_weight {
                1.15 // 15% bonus for diversity contributions
            } else {
                1.0
            };

            let sustainability_multiplier = if input.sustainability_bonus && ai_weight {
                1.10 // 10% bonus for sustainability
            } else {
                1.0
            };

            let total_points = base_points * diversity_multiplier * sustainability_multiplier;

            let points = CCIPoints {
                member_id,
                base_points,
                diversity_multiplier,
                sustainability_multiplier,
                total_points,
            };

            info!("CCI for member {}: {:.2} points", member_id, total_points);

            self.cci_scores.insert(member_id, points.clone());

            // For demo, return first member's points
            return Ok(points);
        }

        Err(anyhow::anyhow!("No contributions found"))
    }

    /// Calculate global residuals distribution
    pub fn global_residuals(
        &self,
        residuals: u128,
        project_id: ProjectId,
        year: u32,
    ) -> Result<Vec<ResidualShare>> {
        info!("Calculating residuals for project {} (year {})", project_id, year);

        // Get all contributors to this project
        let project_contribs: Vec<&Contribution> = self
            .contributions
            .iter()
            .filter(|c| c.project_id == project_id)
            .collect();

        if project_contribs.is_empty() {
            return Ok(Vec::new());
        }

        // Calculate total CCI points for this project
        let total_points: f64 = project_contribs
            .iter()
            .filter_map(|c| self.cci_scores.get(&c.member_id))
            .map(|cci| cci.total_points)
            .sum();

        // Calculate shares
        let mut shares = Vec::new();

        for contribution in project_contribs {
            if let Some(cci) = self.cci_scores.get(&contribution.member_id) {
                let share_percentage = (cci.total_points / total_points) * 100.0;
                let amount = (residuals as f64 * share_percentage / 100.0) as u128;

                shares.push(ResidualShare {
                    member_id: contribution.member_id,
                    project_id,
                    share_percentage,
                    amount,
                });
            }
        }

        info!("Distributed {} in residuals to {} members", residuals, shares.len());

        Ok(shares)
    }

    /// AI-powered bias detection
    pub fn bias_detect(&self, cci: &CCIPoints) -> Result<Vec<BiasAlert>> {
        info!("Running bias detection for member {}", cci.member_id);

        let mut alerts = Vec::new();

        // Check for unusually low diversity multiplier
        if cci.diversity_multiplier < 1.05 {
            alerts.push(BiasAlert {
                timestamp: Utc::now(),
                alert_type: "Low Diversity Representation".to_string(),
                description: "Member may not be receiving diversity bonuses".to_string(),
                severity: AlertSeverity::Medium,
            });
        }

        // Check for zero sustainability bonus
        if cci.sustainability_multiplier == 1.0 {
            alerts.push(BiasAlert {
                timestamp: Utc::now(),
                alert_type: "No Sustainability Bonus".to_string(),
                description: "Consider sustainability-focused contributions".to_string(),
                severity: AlertSeverity::Low,
            });
        }

        if !alerts.is_empty() {
            warn!("Detected {} potential biases", alerts.len());
        }

        Ok(alerts)
    }

    /// Add a contribution
    pub fn add_contribution(&mut self, contribution: Contribution) -> Result<()> {
        self.contributions.push(contribution);
        Ok(())
    }

    /// Get CCI score for a member
    pub fn get_cci_score(&self, member_id: MemberId) -> Option<&CCIPoints> {
        self.cci_scores.get(&member_id)
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

    #[test]
    fn test_cci_calculation() {
        let mut cci_system = CCISystem::new().unwrap();

        let member_id = uuid::Uuid::new_v4();
        let project_id = uuid::Uuid::new_v4();

        let contribution = Contribution {
            member_id,
            project_id,
            contribution_type: ContributionType::Direction,
            hours: 100.0,
            impact_score: 0.8,
            timestamp: Utc::now(),
        };

        let input = CCIInput {
            contributions: vec![contribution],
            diversity_bonus: true,
            sustainability_bonus: true,
        };

        let points = cci_system.intelligent_cci(input, true).unwrap();

        assert!(points.total_points > points.base_points);
    }

    #[test]
    fn test_residuals() {
        let mut cci_system = CCISystem::new().unwrap();
        let member_id = uuid::Uuid::new_v4();
        let project_id = uuid::Uuid::new_v4();

        let contribution = Contribution {
            member_id,
            project_id,
            contribution_type: ContributionType::Acting,
            hours: 50.0,
            impact_score: 0.9,
            timestamp: Utc::now(),
        };

        cci_system.add_contribution(contribution).unwrap();

        let input = CCIInput {
            contributions: cci_system.contributions.clone(),
            diversity_bonus: false,
            sustainability_bonus: false,
        };

        cci_system.intelligent_cci(input, false).unwrap();

        let shares = cci_system.global_residuals(100000, project_id, 1).unwrap();
        assert!(!shares.is_empty());
    }
}
