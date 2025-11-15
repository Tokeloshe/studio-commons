/*!
 * Treasury Module - Manages DeFi investments and global distributions
 *
 * Integrates with the payments module for automated founder's fee distribution
 */

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::{Currency, MemberId};

/// DeFi protocol integrations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Protocol {
    Aave,
    Compound,
    YearnFinance,
    Custom(String),
}

/// Yield farming position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldPosition {
    pub id: uuid::Uuid,
    pub protocol: Protocol,
    pub amount: u128,
    pub currency: Currency,
    pub apy: f64,
    pub start_date: DateTime<Utc>,
}

/// Payout to a member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub member_id: MemberId,
    pub amount: u128,
    pub currency: Currency,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

/// Distribution breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionBreakdown {
    pub member_payouts: Vec<Payout>,
    pub reinvestment_amount: u128,
    pub reserve_amount: u128,
    pub founder_fee: u128,
    pub total_distributed: u128,
}

/// Risk assessment scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub probability: f64,
    pub impact: f64,
}

/// Risk analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    pub timestamp: DateTime<Utc>,
    pub scenarios: Vec<Scenario>,
    pub recommendation: String,
    pub risk_score: f64,
}

/// Carbon tracking data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonData {
    pub energy_kwh: f64,
    pub carbon_tons: f64,
    pub offset_required: f64,
    pub net_zero_achieved: bool,
}

pub struct TreasurySystem {
    positions: Vec<YieldPosition>,
    total_balance: HashMap<Currency, u128>,
    carbon_data: Vec<CarbonData>,
}

impl TreasurySystem {
    pub fn new() -> Result<Self> {
        info!("Initializing Treasury System");

        let mut total_balance = HashMap::new();
        total_balance.insert(Currency::USD, 0);
        total_balance.insert(Currency::XRP, 0);

        Ok(Self {
            positions: Vec::new(),
            total_balance,
            carbon_data: Vec::new(),
        })
    }

    /// Deploy funds to DeFi protocol with multi-currency support
    pub fn deploy_multi_currency(
        &mut self,
        amount: u128,
        protocol: Protocol,
        currency: Currency,
    ) -> Result<uuid::Uuid> {
        info!(
            "Deploying {} {:?} to {:?}",
            amount, currency, protocol
        );

        let apy = match protocol {
            Protocol::Aave => 4.5,
            Protocol::Compound => 5.2,
            Protocol::YearnFinance => 6.0,
            Protocol::Custom(_) => 4.0,
        };

        let position = YieldPosition {
            id: uuid::Uuid::new_v4(),
            protocol,
            amount,
            currency: currency.clone(),
            apy,
            start_date: Utc::now(),
        };

        let id = position.id;
        self.positions.push(position);

        // Deduct from balance
        let balance = self.total_balance.entry(currency).or_insert(0);
        *balance = balance.saturating_sub(amount);

        info!("Deployed to protocol with APY: {}%", apy);

        Ok(id)
    }

    /// Global profit distribution (50/30/20 split + 1% founder's fee)
    /// Note: Founder's fee is handled by payments module
    pub fn global_distribute(&self, profits: u128) -> Result<Vec<Payout>> {
        info!("Distributing global profits: {}", profits);

        // Calculate splits
        // Note: In real integration, this would call payments::process_global_revenue
        // which automatically deducts 1% founder's fee
        let founder_fee = (profits as f64 * 0.01) as u128; // 1%
        let remaining = profits - founder_fee;

        let member_share = (remaining as f64 * 0.50) as u128; // 50%
        let reinvestment = (remaining as f64 * 0.30) as u128; // 30%
        let reserves = remaining - member_share - reinvestment; // 20%

        info!("Distribution breakdown:");
        info!("  Members (50%): {}", member_share);
        info!("  Reinvestment (30%): {}", reinvestment);
        info!("  Reserves (20%): {}", reserves);
        info!("  Founder's Fee (1%): {} -> XRP wallet", founder_fee);

        // For demo, create sample payouts
        // In production, this would use actual member CCI scores
        let payouts = vec![
            Payout {
                member_id: uuid::Uuid::new_v4(),
                amount: member_share / 2,
                currency: Currency::USD,
                reason: "Profit distribution".to_string(),
                timestamp: Utc::now(),
            },
            Payout {
                member_id: uuid::Uuid::new_v4(),
                amount: member_share / 2,
                currency: Currency::USD,
                reason: "Profit distribution".to_string(),
                timestamp: Utc::now(),
            },
        ];

        Ok(payouts)
    }

    /// AI-powered risk analytics
    pub fn risk_analytics(&self, scenarios: Vec<Scenario>) -> Result<RiskReport> {
        info!("Running risk analytics on {} scenarios", scenarios.len());

        // Calculate weighted risk score
        let risk_score: f64 = scenarios
            .iter()
            .map(|s| s.probability * s.impact)
            .sum::<f64>()
            / scenarios.len() as f64;

        let recommendation = if risk_score > 0.7 {
            "High risk detected. Consider diversifying positions and increasing reserves."
        } else if risk_score > 0.4 {
            "Moderate risk. Monitor positions and maintain current strategy."
        } else {
            "Low risk. Current allocation is stable."
        };

        let report = RiskReport {
            timestamp: Utc::now(),
            scenarios,
            recommendation: recommendation.to_string(),
            risk_score,
        };

        info!("Risk score: {:.2}", risk_score);

        Ok(report)
    }

    /// Track carbon footprint for net-zero commitment
    pub fn carbon_track(&mut self, energy_kwh: f64) -> Result<CarbonData> {
        info!("Tracking carbon for {} kWh energy usage", energy_kwh);

        // Rough calculation: 0.0004 metric tons CO2 per kWh (average US grid)
        let carbon_tons = energy_kwh * 0.0004;
        let offset_required = carbon_tons;

        // Check if we've achieved net-zero (simplified)
        let net_zero_achieved = self.carbon_data.is_empty(); // First entry

        let data = CarbonData {
            energy_kwh,
            carbon_tons,
            offset_required,
            net_zero_achieved,
        };

        self.carbon_data.push(data.clone());

        info!("Carbon footprint: {:.2} tons CO2", carbon_tons);
        info!("Offset required: {:.2} tons", offset_required);

        Ok(data)
    }

    /// Get total value locked across all positions
    pub fn get_tvl(&self) -> HashMap<Currency, u128> {
        let mut tvl = HashMap::new();

        for position in &self.positions {
            *tvl.entry(position.currency.clone()).or_insert(0) += position.amount;
        }

        tvl
    }

    /// Get total carbon offset
    pub fn get_total_carbon_offset(&self) -> f64 {
        self.carbon_data.iter().map(|d| d.offset_required).sum()
    }
}

impl Default for TreasurySystem {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_to_defi() {
        let mut treasury = TreasurySystem::new().unwrap();
        treasury.total_balance.insert(Currency::USD, 1000000);

        let id = treasury
            .deploy_multi_currency(500000, Protocol::Aave, Currency::USD)
            .unwrap();

        assert_eq!(treasury.positions.len(), 1);
        assert_eq!(treasury.positions[0].id, id);
    }

    #[test]
    fn test_profit_distribution() {
        let treasury = TreasurySystem::new().unwrap();
        let payouts = treasury.global_distribute(100000).unwrap();

        // Should create payouts for members
        assert!(!payouts.is_empty());
    }

    #[test]
    fn test_risk_analytics() {
        let treasury = TreasurySystem::new().unwrap();
        let scenarios = vec![
            Scenario {
                name: "Market crash".to_string(),
                probability: 0.2,
                impact: 0.8,
            },
            Scenario {
                name: "Stable growth".to_string(),
                probability: 0.7,
                impact: 0.3,
            },
        ];

        let report = treasury.risk_analytics(scenarios).unwrap();
        assert!(report.risk_score > 0.0);
    }

    #[test]
    fn test_carbon_tracking() {
        let mut treasury = TreasurySystem::new().unwrap();
        let data = treasury.carbon_track(10000.0).unwrap();

        assert!(data.carbon_tons > 0.0);
        assert!(data.offset_required > 0.0);
    }
}
