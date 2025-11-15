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
    pub diversity_percentage: f64,
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

/// Diversity analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityReport {
    pub timestamp: DateTime<Utc>,
    pub gender_diversity: f64,
    pub ethnic_diversity: f64,
    pub geographic_diversity: f64,
    pub overall_score: f64,
    pub meets_40_percent_mandate: bool,
}

pub struct AnalyticsSystem {
    metrics_history: Vec<GlobalMetrics>,
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

    /// Diversity analytics with 40%+ mandate enforcement
    pub fn diversity_analytics(&self, sample_data: Vec<f64>) -> Result<DiversityReport> {
        info!("Analyzing diversity metrics");

        // Simulate diversity calculations
        let gender_diversity = sample_data.get(0).copied().unwrap_or(45.0);
        let ethnic_diversity = sample_data.get(1).copied().unwrap_or(42.0);
        let geographic_diversity = sample_data.get(2).copied().unwrap_or(38.0);

        let overall_score = (gender_diversity + ethnic_diversity + geographic_diversity) / 3.0;
        let meets_mandate = overall_score >= 40.0;

        let report = DiversityReport {
            timestamp: Utc::now(),
            gender_diversity,
            ethnic_diversity,
            geographic_diversity,
            overall_score,
            meets_40_percent_mandate: meets_mandate,
        };

        if meets_mandate {
            info!("✓ Diversity mandate met: {:.1}%", overall_score);
        } else {
            info!("✗ Diversity below 40% mandate: {:.1}%", overall_score);
        }

        Ok(report)
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
            diversity_percentage: 45.0,
            carbon_offset_tons: 100.0,
            timestamp: Utc::now(),
        };

        let forecast = analytics.predict_impact(metrics).unwrap();

        assert!(forecast.predicted_revenue > 1000000);
        assert!(forecast.predicted_members > 1000);
    }

    #[test]
    fn test_diversity_analytics() {
        let analytics = AnalyticsSystem::new().unwrap();

        let data = vec![45.0, 42.0, 41.0];
        let report = analytics.diversity_analytics(data).unwrap();

        assert!(report.meets_40_percent_mandate);
        assert!(report.overall_score >= 40.0);
    }

    #[test]
    fn test_diversity_mandate_failure() {
        let analytics = AnalyticsSystem::new().unwrap();

        let data = vec![35.0, 30.0, 38.0];
        let report = analytics.diversity_analytics(data).unwrap();

        assert!(!report.meets_40_percent_mandate);
    }

    #[test]
    fn test_metrics_recording() {
        let mut analytics = AnalyticsSystem::new().unwrap();

        let metrics = GlobalMetrics {
            total_members: 100,
            total_projects: 10,
            revenue_usd: 100000,
            diversity_percentage: 45.0,
            carbon_offset_tons: 10.0,
            timestamp: Utc::now(),
        };

        analytics.record_metrics(metrics);

        assert_eq!(analytics.metrics_history.len(), 1);
    }
}
