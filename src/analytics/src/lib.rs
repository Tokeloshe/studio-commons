/*!
 * Analytics Module - Predictive intelligence and impact forecasting
 */

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};

/// Global metrics for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMetrics {
    pub total_members: usize,
    pub total_projects: usize,
    pub revenue_usd: u128,
    pub carbon_offset_tons: f64,
    pub timestamp: DateTime<Utc>,
}

/// Impact forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactForecast {
    pub year: u32,
    pub predicted_revenue: u128,
    pub predicted_members: usize,
    pub predicted_projects: usize,
    pub economic_impact: f64,
    pub cultural_impact_score: f64,
    pub confidence_level: f64,
}

/// Identity-blind fairness report.
///
/// Instead of tracking who members are, this measures whether the system's
/// *outcomes* are healthy: are rewards spread in proportion to contributions,
/// or captured by a small clique? The Gini coefficient over earned merit
/// points answers that without a single demographic data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FairnessReport {
    pub timestamp: DateTime<Utc>,
    /// Gini coefficient of merit-point distribution: 0 = perfectly even,
    /// 1 = one member captures everything.
    pub reward_concentration: f64,
    /// Fraction of members whose earned share is nonzero.
    pub participation_rate: f64,
    /// Flags possible capture when concentration exceeds the threshold.
    pub capture_warning: bool,
}

/// Concentration above this suggests rewards are pooling in a small group —
/// a signal for members to audit the ledger, not an automatic judgment.
pub const CAPTURE_THRESHOLD: f64 = 0.8;

pub struct AnalyticsSystem {
    metrics_history: Vec<GlobalMetrics>,
}

/// Gini coefficient of a non-negative distribution.
fn gini(values: &[f64]) -> f64 {
    let n = values.len();
    if n == 0 {
        return 0.0;
    }
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let total: f64 = sorted.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let weighted: f64 = sorted
        .iter()
        .enumerate()
        .map(|(i, v)| (i as f64 + 1.0) * v)
        .sum();
    (2.0 * weighted) / (n as f64 * total) - (n as f64 + 1.0) / n as f64
}

impl AnalyticsSystem {
    pub fn new() -> Result<Self> {
        info!("Initializing Analytics System");

        Ok(Self {
            metrics_history: Vec::new(),
        })
    }

    /// AI-powered impact prediction
    pub fn predict_impact(&self, metrics: GlobalMetrics) -> Result<ImpactForecast> {
        info!("Generating impact forecast based on current metrics");

        // Simple growth model (in production, this would use ML)
        let growth_rate = 1.25; // 25% annual growth
        let current_year = 2025;

        let forecast = ImpactForecast {
            year: current_year + 1,
            predicted_revenue: (metrics.revenue_usd as f64 * growth_rate) as u128,
            predicted_members: (metrics.total_members as f64 * growth_rate) as usize,
            predicted_projects: (metrics.total_projects as f64 * growth_rate) as usize,
            economic_impact: metrics.revenue_usd as f64 * 2.5, // Economic multiplier
            cultural_impact_score: 85.0, // Simulated score
            confidence_level: 0.78,
        };

        info!("Forecast: ${} revenue, {} members, {} projects",
              forecast.predicted_revenue,
              forecast.predicted_members,
              forecast.predicted_projects);

        Ok(forecast)
    }

    /// Fairness analytics over merit-point totals per member.
    /// Takes each member's earned points; identities never enter the math.
    pub fn fairness_analytics(&self, member_points: Vec<f64>) -> Result<FairnessReport> {
        info!("Analyzing reward-distribution fairness");

        let reward_concentration = gini(&member_points);
        let participation_rate = if member_points.is_empty() {
            0.0
        } else {
            member_points.iter().filter(|p| **p > 0.0).count() as f64
                / member_points.len() as f64
        };
        let capture_warning = reward_concentration > CAPTURE_THRESHOLD;

        if capture_warning {
            info!(
                "✗ Reward concentration {:.2} exceeds capture threshold — audit recommended",
                reward_concentration
            );
        } else {
            info!("✓ Reward concentration healthy: {:.2}", reward_concentration);
        }

        Ok(FairnessReport {
            timestamp: Utc::now(),
            reward_concentration,
            participation_rate,
            capture_warning,
        })
    }

    /// Record metrics snapshot
    pub fn record_metrics(&mut self, metrics: GlobalMetrics) {
        info!("Recording metrics snapshot");
        self.metrics_history.push(metrics);
    }

    /// Get historical trends
    pub fn get_trends(&self, last_n: usize) -> Vec<&GlobalMetrics> {
        let len = self.metrics_history.len();
        if len <= last_n {
            self.metrics_history.iter().collect()
        } else {
            self.metrics_history[len - last_n..].iter().collect()
        }
    }

    /// Calculate growth rate
    pub fn calculate_growth_rate(&self) -> Option<f64> {
        if self.metrics_history.len() < 2 {
            return None;
        }

        let first = &self.metrics_history[0];
        let last = &self.metrics_history[self.metrics_history.len() - 1];

        let revenue_growth = (last.revenue_usd as f64 - first.revenue_usd as f64)
                           / first.revenue_usd as f64 * 100.0;

        Some(revenue_growth)
    }
}

impl Default for AnalyticsSystem {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impact_prediction() {
        let analytics = AnalyticsSystem::new().unwrap();

        let metrics = GlobalMetrics {
            total_members: 1000,
            total_projects: 50,
            revenue_usd: 1000000,
            carbon_offset_tons: 100.0,
            timestamp: Utc::now(),
        };

        let forecast = analytics.predict_impact(metrics).unwrap();

        assert!(forecast.predicted_revenue > 1000000);
        assert!(forecast.predicted_members > 1000);
    }

    #[test]
    fn test_fairness_healthy_distribution() {
        let analytics = AnalyticsSystem::new().unwrap();

        // Rewards roughly proportional to varied contribution levels.
        let points = vec![80.0, 120.0, 100.0, 90.0, 110.0];
        let report = analytics.fairness_analytics(points).unwrap();

        assert!(!report.capture_warning);
        assert!(report.participation_rate > 0.99);
    }

    #[test]
    fn test_fairness_capture_detected() {
        let analytics = AnalyticsSystem::new().unwrap();

        // One member captures nearly everything.
        let points = vec![10_000.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let report = analytics.fairness_analytics(points).unwrap();

        assert!(report.capture_warning);
        assert!(report.reward_concentration > CAPTURE_THRESHOLD);
    }

    #[test]
    fn test_metrics_recording() {
        let mut analytics = AnalyticsSystem::new().unwrap();

        let metrics = GlobalMetrics {
            total_members: 100,
            total_projects: 10,
            revenue_usd: 100000,
            carbon_offset_tons: 10.0,
            timestamp: Utc::now(),
        };

        analytics.record_metrics(metrics);

        assert_eq!(analytics.metrics_history.len(), 1);
    }
}
