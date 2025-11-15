/*!
 * Payments Module - Handles all revenue processing and distributions
 *
 * CRITICAL: This module contains hardcoded founder's fee configuration
 * - XRP Wallet: rf82s1CDagppvM6ATqc1nSrL6GackzHJrm
 * - Memo: 2621443948
 * - Fee: 1% of net profits (perpetual, immutable)
 */

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::{Currency, MemberId, ProjectId, StreamType, TransactionHash};

/// IMMUTABLE FOUNDER'S FEE CONFIGURATION
/// This 1% fee supports the ongoing vision and development of Studio Commons
pub const FOUNDER_XRP_WALLET: &str = "rf82s1CDagppvM6ATqc1nSrL6GackzHJrm";
pub const FOUNDER_XRP_MEMO: &str = "2621443948";
pub const FOUNDER_FEE_PERCENTAGE: f64 = 1.0; // 1% of net profits

/// Revenue allocation percentages
pub const MEMBER_DISTRIBUTION_PERCENTAGE: f64 = 50.0;
pub const REINVESTMENT_PERCENTAGE: f64 = 30.0;
pub const RESERVE_PERCENTAGE: f64 = 20.0;

/// Payment allocation breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    pub member_distribution: u128,
    pub reinvestment: u128,
    pub reserves: u128,
    pub founder_fee: u128,
    pub total_processed: u128,
    pub timestamp: DateTime<Utc>,
}

/// XRP transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XRPTransaction {
    pub wallet_address: String,
    pub memo: String,
    pub amount: u128,
    pub currency: Currency,
    pub tx_hash: Option<TransactionHash>,
    pub timestamp: DateTime<Utc>,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

/// Individual payout to a member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub member_id: MemberId,
    pub amount: u128,
    pub currency: Currency,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

pub struct PaymentsSystem {
    transactions: Vec<XRPTransaction>,
    allocations: Vec<Allocation>,
}

impl PaymentsSystem {
    pub fn new() -> Result<Self> {
        info!("Initializing Payments System");
        info!("Founder's Fee: {}% to wallet {}", FOUNDER_FEE_PERCENTAGE, FOUNDER_XRP_WALLET);
        info!("Memo: {}", FOUNDER_XRP_MEMO);

        Ok(Self {
            transactions: Vec::new(),
            allocations: Vec::new(),
        })
    }

    /// Process global revenue from various streams
    /// Automatically calculates and allocates founder's fee
    pub fn process_global_revenue(
        &mut self,
        stream: StreamType,
        amount: u128,
        currency: Currency,
    ) -> Result<Allocation> {
        info!("Processing revenue: {:?} {} {:?}", amount, stream, currency);

        // Calculate allocations based on net profits
        let founder_fee = (amount as f64 * FOUNDER_FEE_PERCENTAGE / 100.0) as u128;
        let remaining = amount - founder_fee;

        let member_distribution = (remaining as f64 * MEMBER_DISTRIBUTION_PERCENTAGE / 100.0) as u128;
        let reinvestment = (remaining as f64 * REINVESTMENT_PERCENTAGE / 100.0) as u128;
        let reserves = remaining - member_distribution - reinvestment;

        let allocation = Allocation {
            member_distribution,
            reinvestment,
            reserves,
            founder_fee,
            total_processed: amount,
            timestamp: Utc::now(),
        };

        // Process founder's fee payment
        self.perpetual_founder_fee(founder_fee, currency.clone())?;

        self.allocations.push(allocation.clone());

        info!("Revenue allocated:");
        info!("  Members: {}", member_distribution);
        info!("  Reinvestment: {}", reinvestment);
        info!("  Reserves: {}", reserves);
        info!("  Founder's Fee (1%): {} -> {}", founder_fee, FOUNDER_XRP_WALLET);

        Ok(allocation)
    }

    /// Execute perpetual founder's fee payment to XRP wallet
    /// This is IMMUTABLE and runs on every profit distribution
    pub fn perpetual_founder_fee(
        &mut self,
        profit: u128,
        currency: Currency,
    ) -> Result<TransactionHash> {
        if profit == 0 {
            warn!("Zero profit, skipping founder's fee");
            return Ok("SKIPPED_ZERO_AMOUNT".to_string());
        }

        let fee_amount = (profit as f64 * FOUNDER_FEE_PERCENTAGE / 100.0) as u128;

        info!("Executing perpetual founder's fee:");
        info!("  Profit: {}", profit);
        info!("  Fee (1%): {}", fee_amount);
        info!("  Destination: {}", FOUNDER_XRP_WALLET);
        info!("  Memo: {}", FOUNDER_XRP_MEMO);

        let transaction = XRPTransaction {
            wallet_address: FOUNDER_XRP_WALLET.to_string(),
            memo: FOUNDER_XRP_MEMO.to_string(),
            amount: fee_amount,
            currency,
            tx_hash: Some(format!("XRP_TX_{}", uuid::Uuid::new_v4())),
            timestamp: Utc::now(),
            status: TransactionStatus::Pending,
        };

        let tx_hash = transaction.tx_hash.clone().unwrap();
        self.transactions.push(transaction);

        // In production, this would interface with XRPL SDK
        // For now, we log and record the intent
        info!("✓ Founder's fee transaction queued: {}", tx_hash);

        Ok(tx_hash)
    }

    /// Distribute payments to members based on CCI scores
    pub fn distribute_to_members(
        &self,
        total_amount: u128,
        member_shares: HashMap<MemberId, f64>,
        currency: Currency,
    ) -> Result<Vec<Payout>> {
        let mut payouts = Vec::new();

        for (member_id, share) in member_shares.iter() {
            let amount = (total_amount as f64 * share) as u128;

            payouts.push(Payout {
                member_id: *member_id,
                amount,
                currency: currency.clone(),
                reason: "CCI-based profit distribution".to_string(),
                timestamp: Utc::now(),
            });
        }

        info!("Distributed {} to {} members", total_amount, payouts.len());

        Ok(payouts)
    }

    /// Get all founder's fee transactions
    pub fn get_founder_transactions(&self) -> Vec<&XRPTransaction> {
        self.transactions
            .iter()
            .filter(|tx| tx.wallet_address == FOUNDER_XRP_WALLET)
            .collect()
    }

    /// Get total founder's fees paid
    pub fn get_total_founder_fees(&self) -> u128 {
        self.transactions
            .iter()
            .filter(|tx| tx.wallet_address == FOUNDER_XRP_WALLET)
            .map(|tx| tx.amount)
            .sum()
    }

    /// Verify founder's fee configuration (for auditing)
    pub fn verify_founder_config() -> (String, String, f64) {
        (
            FOUNDER_XRP_WALLET.to_string(),
            FOUNDER_XRP_MEMO.to_string(),
            FOUNDER_FEE_PERCENTAGE,
        )
    }
}

impl Default for PaymentsSystem {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_founder_config_immutable() {
        assert_eq!(FOUNDER_XRP_WALLET, "rf82s1CDagppvM6ATqc1nSrL6GackzHJrm");
        assert_eq!(FOUNDER_XRP_MEMO, "2621443948");
        assert_eq!(FOUNDER_FEE_PERCENTAGE, 1.0);
    }

    #[test]
    fn test_founder_fee_calculation() {
        let mut system = PaymentsSystem::new().unwrap();
        let profit = 100000; // $1000.00
        let tx_hash = system.perpetual_founder_fee(profit, Currency::USD).unwrap();

        assert!(tx_hash.contains("XRP_TX_"));
        assert_eq!(system.get_total_founder_fees(), 1000); // 1% of 100000
    }

    #[test]
    fn test_revenue_allocation() {
        let mut system = PaymentsSystem::new().unwrap();
        let allocation = system
            .process_global_revenue(StreamType::Rental, 100000, Currency::USD)
            .unwrap();

        // 1% founder fee = 1000
        // Remaining 99000: 50% members, 30% reinvest, 20% reserves
        assert_eq!(allocation.founder_fee, 1000);
        assert_eq!(allocation.member_distribution, 49500);
        assert_eq!(allocation.reinvestment, 29700);
        // reserves should be the remainder
    }

    #[test]
    fn test_verify_founder_config() {
        let (wallet, memo, fee) = PaymentsSystem::verify_founder_config();
        assert_eq!(wallet, "rf82s1CDagppvM6ATqc1nSrL6GackzHJrm");
        assert_eq!(memo, "2621443948");
        assert_eq!(fee, 1.0);
    }

    #[test]
    fn test_get_founder_transactions() {
        let mut system = PaymentsSystem::new().unwrap();
        system.perpetual_founder_fee(100000, Currency::USD).unwrap();
        system.perpetual_founder_fee(50000, Currency::EUR).unwrap();

        let founder_txs = system.get_founder_transactions();
        assert_eq!(founder_txs.len(), 2);
        assert!(founder_txs.iter().all(|tx| tx.wallet_address == FOUNDER_XRP_WALLET));
    }
}
