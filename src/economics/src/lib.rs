/*!
 * Economics Module - the fiscal engine that makes a hub sustainable,
 * profitable, and capable of funding its own expansion.
 *
 * Design principles:
 *
 * - Costs come first. Nothing is "profit" until every operating expense of
 *   the period is covered. Distributing gross revenue is how co-ops die;
 *   this engine makes it structurally impossible.
 * - Exact integer arithmetic. All money is u128 in minor units and all
 *   percentages are basis points. Every period closes with an exact
 *   conservation identity: revenue = expenses + fee + distributions +
 *   fund deposits. Not approximately — exactly, provable by audit().
 * - Survival before generosity. Allocation follows the hub's runway
 *   (months of reserves at current burn) through three states:
 *     Critical    (< 3 months): every unit of surplus rebuilds reserves.
 *     Rebuilding  (3-6 months): half rebuilds reserves, the rest flows on.
 *     Healthy     (>= 6 months): full member distribution resumes.
 * - Expansion is earned, not hoped for. A dedicated expansion fund grows
 *   only from genuine surplus, and a new hub can only be seeded after a
 *   sustained track record of profitable, healthy periods. Growth can
 *   never endanger the hub that funds it.
 */

use anyhow::{bail, Result};
use log::info;
use serde::{Deserialize, Serialize};

/// All rates are in basis points; 10_000 bps = 100%.
pub const BPS: u128 = 10_000;

/// Founder's fee: 1% of net surplus (net profit), per the platform charter.
/// Levied only when a period actually ends in surplus — a loss-making
/// period pays no fee, because there is no profit to take 1% of.
pub const FOUNDER_FEE_BPS: u128 = 100;

/// Healthy-state split of post-fee surplus.
pub const MEMBER_BPS: u128 = 5_000; // 50% to members by CCI merit
pub const REINVEST_BPS: u128 = 2_000; // 20% local reinvestment
pub const EXPANSION_BPS: u128 = 1_000; // 10% to the global expansion fund
// Reserves receive the remainder (~20%), absorbing all rounding dust so
// conservation is exact.

/// Runway thresholds, in months of average operating expenses.
pub const MIN_RUNWAY_MONTHS: u128 = 3;
pub const TARGET_RUNWAY_MONTHS: u128 = 6;
/// Reserves above this cap overflow into the expansion fund: money beyond
/// prudence should build the next hub, not sit idle.
pub const RESERVE_CAP_MONTHS: u128 = 12;

/// Consecutive profitable, healthy periods required before this hub may
/// seed a new one.
pub const SEED_TRACK_RECORD_PERIODS: usize = 3;

/// Fiscal health of the hub, derived purely from runway.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FiscalHealth {
    /// Reserves exhausted and obligations unpaid.
    Insolvent,
    /// Runway below MIN_RUNWAY_MONTHS.
    Critical,
    /// Runway between MIN and TARGET.
    Rebuilding,
    /// Runway at or above TARGET.
    Healthy,
}

/// Result of closing one fiscal period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodReport {
    pub period_index: usize,
    pub revenue: u128,
    pub expenses: u128,
    /// revenue - expenses when positive, else 0.
    pub surplus: u128,
    /// expenses - revenue when positive, else 0.
    pub deficit: u128,
    /// Deficit covered by drawing down reserves.
    pub reserves_drawn: u128,
    /// Deficit that could not be covered — the hub is insolvent.
    pub unfunded_deficit: u128,
    /// Past unfunded obligations settled out of this period's gross surplus,
    /// before anything counted as profit.
    pub debt_repaid: u128,
    pub founder_fee: u128,
    pub member_distribution: u128,
    pub local_reinvestment: u128,
    pub expansion_deposit: u128,
    pub reserve_deposit: u128,
    /// Reserves above the prudence cap moved into the expansion fund. This
    /// is a transfer between held balances — possibly of money earned in
    /// prior periods — so it is reported separately and never counted as
    /// part of this period's surplus allocation.
    pub reserve_overflow_to_expansion: u128,
    pub reserves_after: u128,
    pub expansion_fund_after: u128,
    pub health_after: FiscalHealth,
}

/// Grant released to seed a new hub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSeed {
    pub id: uuid::Uuid,
    pub amount: u128,
    pub funded_after_period: usize,
}

/// Conservation audit: every unit in must equal every unit out or held.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationAudit {
    pub lifetime_revenue: u128,
    pub lifetime_expenses_paid: u128,
    pub lifetime_founder_fees: u128,
    pub lifetime_member_distributions: u128,
    pub lifetime_reinvestment: u128,
    pub lifetime_hub_seeds: u128,
    pub reserves_held: u128,
    pub expansion_fund_held: u128,
    pub unfunded_deficit: u128,
    /// True iff in == out + held, exactly.
    pub conserved: bool,
}

pub struct FiscalEngine {
    reserves: u128,
    expansion_fund: u128,
    open_revenue: u128,
    open_expenses: u128,
    reports: Vec<PeriodReport>,
    seeds: Vec<HubSeed>,
    // Lifetime counters for the conservation identity.
    lifetime_revenue: u128,
    lifetime_expenses_paid: u128,
    lifetime_founder_fees: u128,
    lifetime_member_distributions: u128,
    lifetime_reinvestment: u128,
    lifetime_hub_seeds: u128,
    unfunded_deficit: u128,
}

fn bps_of(amount: u128, bps: u128) -> u128 {
    // u128 * u128 can overflow only near 2^114 minor units — beyond any
    // plausible economy — but checked math costs nothing.
    amount
        .checked_mul(bps)
        .map(|x| x / BPS)
        .unwrap_or_else(|| (amount / BPS) * bps)
}

impl FiscalEngine {
    pub fn new() -> Result<Self> {
        info!("Initializing Fiscal Engine (costs-first, integer-exact)");

        Ok(Self {
            reserves: 0,
            expansion_fund: 0,
            open_revenue: 0,
            open_expenses: 0,
            reports: Vec::new(),
            seeds: Vec::new(),
            lifetime_revenue: 0,
            lifetime_expenses_paid: 0,
            lifetime_founder_fees: 0,
            lifetime_member_distributions: 0,
            lifetime_reinvestment: 0,
            lifetime_hub_seeds: 0,
            unfunded_deficit: 0,
        })
    }

    /// Record revenue for the open period (minor units).
    pub fn record_revenue(&mut self, amount: u128) -> Result<()> {
        self.open_revenue = self
            .open_revenue
            .checked_add(amount)
            .ok_or_else(|| anyhow::anyhow!("revenue overflow"))?;
        self.lifetime_revenue = self
            .lifetime_revenue
            .checked_add(amount)
            .ok_or_else(|| anyhow::anyhow!("revenue overflow"))?;
        Ok(())
    }

    /// Record an operating expense for the open period (minor units).
    pub fn record_expense(&mut self, amount: u128) -> Result<()> {
        self.open_expenses = self
            .open_expenses
            .checked_add(amount)
            .ok_or_else(|| anyhow::anyhow!("expense overflow"))?;
        Ok(())
    }

    /// Average monthly operating expenses over the trailing year of closed
    /// periods (or the open period if none closed yet). This is the burn
    /// rate that runway is measured against.
    pub fn average_monthly_expenses(&self) -> u128 {
        let recent: Vec<u128> = self
            .reports
            .iter()
            .rev()
            .take(12)
            .map(|r| r.expenses)
            .collect();
        if recent.is_empty() {
            return self.open_expenses;
        }
        recent.iter().sum::<u128>() / recent.len() as u128
    }

    /// Months of runway: how long reserves last at the current burn rate.
    /// Infinite burn-free hubs report u128::MAX.
    pub fn runway_months(&self) -> u128 {
        let burn = self.average_monthly_expenses();
        if burn == 0 {
            return u128::MAX;
        }
        self.reserves / burn
    }

    pub fn health(&self) -> FiscalHealth {
        if self.unfunded_deficit > 0 {
            return FiscalHealth::Insolvent;
        }
        let runway = self.runway_months();
        if runway < MIN_RUNWAY_MONTHS {
            FiscalHealth::Critical
        } else if runway < TARGET_RUNWAY_MONTHS {
            FiscalHealth::Rebuilding
        } else {
            FiscalHealth::Healthy
        }
    }

    /// Close the open period and run the allocation waterfall.
    ///
    /// Deficit path: draw reserves; anything reserves cannot cover is an
    /// unfunded deficit and the hub is insolvent. No fee, no distributions —
    /// there is no profit.
    ///
    /// Surplus path: founder fee (1% of surplus), then allocation by health:
    /// - Critical:   100% of post-fee surplus to reserves.
    /// - Rebuilding: 50% to reserves; the rest split half members / half
    ///               local reinvestment.
    /// - Healthy:    50% members, 20% reinvestment, 10% expansion,
    ///               remainder (~20% + dust) to reserves.
    /// Reserves above RESERVE_CAP_MONTHS overflow into the expansion fund.
    pub fn close_period(&mut self) -> Result<PeriodReport> {
        let revenue = self.open_revenue;
        let expenses = self.open_expenses;
        // Judge health BEFORE this period's results land: a struggling hub
        // cannot pay out on the strength of one good month. (With no closed
        // history, burn falls back to the open period's own expenses.)
        let pre_health = self.health();
        self.open_revenue = 0;
        self.open_expenses = 0;

        let period_index = self.reports.len();
        info!(
            "Closing period {}: revenue {}, expenses {}",
            period_index, revenue, expenses
        );

        let mut report = PeriodReport {
            period_index,
            revenue,
            expenses,
            surplus: 0,
            deficit: 0,
            reserves_drawn: 0,
            unfunded_deficit: 0,
            debt_repaid: 0,
            founder_fee: 0,
            member_distribution: 0,
            local_reinvestment: 0,
            expansion_deposit: 0,
            reserve_deposit: 0,
            reserve_overflow_to_expansion: 0,
            reserves_after: 0,
            expansion_fund_after: 0,
            health_after: FiscalHealth::Critical,
        };

        if revenue >= expenses {
            let mut surplus = revenue - expenses;
            self.lifetime_expenses_paid += expenses;

            // Debts before profit: unfunded obligations from past periods
            // are settled first. A hub cannot call anything "surplus" while
            // its past bills remain unpaid — this is what lets an insolvent
            // hub genuinely recover instead of paying out around its debt.
            let repaid = surplus.min(self.unfunded_deficit);
            surplus -= repaid;
            self.unfunded_deficit -= repaid;
            self.lifetime_expenses_paid += repaid;
            report.debt_repaid = repaid;

            report.surplus = surplus;

            let fee = bps_of(surplus, FOUNDER_FEE_BPS);
            let post_fee = surplus - fee;
            report.founder_fee = fee;
            self.lifetime_founder_fees += fee;

            let (members, reinvest, expansion, to_reserves) = match pre_health {
                FiscalHealth::Insolvent | FiscalHealth::Critical => (0, 0, 0, post_fee),
                FiscalHealth::Rebuilding => {
                    let to_reserves = bps_of(post_fee, 5_000);
                    let flow_on = post_fee - to_reserves;
                    let members = flow_on / 2;
                    let reinvest = flow_on - members;
                    (members, reinvest, 0, to_reserves)
                }
                FiscalHealth::Healthy => {
                    let members = bps_of(post_fee, MEMBER_BPS);
                    let reinvest = bps_of(post_fee, REINVEST_BPS);
                    let expansion = bps_of(post_fee, EXPANSION_BPS);
                    // Remainder — including every unit of rounding dust —
                    // lands in reserves, keeping conservation exact.
                    let to_reserves = post_fee - members - reinvest - expansion;
                    (members, reinvest, expansion, to_reserves)
                }
            };

            report.member_distribution = members;
            report.local_reinvestment = reinvest;
            report.expansion_deposit = expansion;
            report.reserve_deposit = to_reserves;

            self.lifetime_member_distributions += members;
            self.lifetime_reinvestment += reinvest;
            self.expansion_fund += expansion;
            self.reserves += to_reserves;

            // Overflow prudence into growth.
            let burn = self.average_monthly_expenses().max(expenses);
            if burn > 0 {
                let cap = burn.saturating_mul(RESERVE_CAP_MONTHS);
                if self.reserves > cap {
                    let overflow = self.reserves - cap;
                    self.reserves = cap;
                    self.expansion_fund += overflow;
                    report.reserve_overflow_to_expansion = overflow;
                }
            }
        } else {
            let deficit = expenses - revenue;
            report.deficit = deficit;

            let drawn = deficit.min(self.reserves);
            self.reserves -= drawn;
            report.reserves_drawn = drawn;
            // Revenue plus reserve draw both go to paying expenses.
            self.lifetime_expenses_paid += revenue + drawn;

            let unfunded = deficit - drawn;
            report.unfunded_deficit = unfunded;
            self.unfunded_deficit += unfunded;
        }

        report.reserves_after = self.reserves;
        report.expansion_fund_after = self.expansion_fund;
        report.health_after = self.health();
        self.reports.push(report.clone());

        info!(
            "Period {} closed: health {:?}, reserves {}, expansion fund {}",
            period_index, report.health_after, self.reserves, self.expansion_fund
        );

        Ok(report)
    }

    /// Seed a new hub from the expansion fund.
    ///
    /// Requirements, all enforced in code:
    /// - the expansion fund fully covers the seed (reserves are never touched)
    /// - the hub is currently Healthy
    /// - the last SEED_TRACK_RECORD_PERIODS closed periods were all
    ///   profitable and Healthy — one good month is not a track record.
    pub fn try_seed_hub(&mut self, seed_cost: u128) -> Result<HubSeed> {
        if seed_cost == 0 {
            bail!("seed cost must be positive");
        }
        if self.health() != FiscalHealth::Healthy {
            bail!(
                "hub is {:?}; expansion requires Healthy (>= {} months runway)",
                self.health(),
                TARGET_RUNWAY_MONTHS
            );
        }
        if self.reports.len() < SEED_TRACK_RECORD_PERIODS {
            bail!(
                "need {} closed periods of track record, have {}",
                SEED_TRACK_RECORD_PERIODS,
                self.reports.len()
            );
        }
        let record = &self.reports[self.reports.len() - SEED_TRACK_RECORD_PERIODS..];
        if !record
            .iter()
            .all(|r| r.surplus > 0 && r.health_after == FiscalHealth::Healthy)
        {
            bail!(
                "last {} periods must all be profitable and Healthy",
                SEED_TRACK_RECORD_PERIODS
            );
        }
        if seed_cost > self.expansion_fund {
            bail!(
                "expansion fund {} cannot cover seed cost {}",
                self.expansion_fund,
                seed_cost
            );
        }

        self.expansion_fund -= seed_cost;
        self.lifetime_hub_seeds += seed_cost;

        let seed = HubSeed {
            id: uuid::Uuid::new_v4(),
            amount: seed_cost,
            funded_after_period: self.reports.len(),
        };
        info!(
            "Seeded new hub {} with {} (expansion fund now {})",
            seed.id, seed.amount, self.expansion_fund
        );
        self.seeds.push(seed.clone());

        Ok(seed)
    }

    /// Exact conservation audit. Every unit of lifetime revenue must be
    /// accounted for as expenses paid, fees, distributions, seeds, or money
    /// still held. Reserve draws move money from `held` to `expenses paid`,
    /// so the identity holds through deficits too.
    pub fn audit(&self) -> ConservationAudit {
        let out = self.lifetime_expenses_paid
            + self.lifetime_founder_fees
            + self.lifetime_member_distributions
            + self.lifetime_reinvestment
            + self.lifetime_hub_seeds;
        let held = self.reserves + self.expansion_fund;
        // Open-period revenue not yet closed is still "held" as cash.
        // Reserve draws cancel exactly: they raise expenses-paid by the same
        // amount they lower held reserves. Seeds likewise: they raise the
        // seed counter by what they remove from the expansion fund.
        let conserved = self.lifetime_revenue == out + held + self.open_revenue;

        ConservationAudit {
            lifetime_revenue: self.lifetime_revenue,
            lifetime_expenses_paid: self.lifetime_expenses_paid,
            lifetime_founder_fees: self.lifetime_founder_fees,
            lifetime_member_distributions: self.lifetime_member_distributions,
            lifetime_reinvestment: self.lifetime_reinvestment,
            lifetime_hub_seeds: self.lifetime_hub_seeds,
            reserves_held: self.reserves,
            expansion_fund_held: self.expansion_fund,
            unfunded_deficit: self.unfunded_deficit,
            conserved,
        }
    }

    pub fn reserves(&self) -> u128 {
        self.reserves
    }

    pub fn expansion_fund(&self) -> u128 {
        self.expansion_fund
    }

    pub fn reports(&self) -> &[PeriodReport] {
        &self.reports
    }

    pub fn seeds(&self) -> &[HubSeed] {
        &self.seeds
    }
}

impl Default for FiscalEngine {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_costs_come_before_any_distribution() {
        let mut engine = FiscalEngine::new().unwrap();
        engine.record_revenue(100_000).unwrap();
        engine.record_expense(100_000).unwrap();
        let report = engine.close_period().unwrap();

        assert_eq!(report.surplus, 0);
        assert_eq!(report.founder_fee, 0);
        assert_eq!(report.member_distribution, 0);
    }

    #[test]
    fn test_deficit_pays_no_fee_and_draws_reserves() {
        let mut engine = FiscalEngine::new().unwrap();
        // Build reserves with a strong first period.
        engine.record_revenue(1_000_000).unwrap();
        engine.record_expense(100_000).unwrap();
        engine.close_period().unwrap();
        let reserves_before = engine.reserves();
        assert!(reserves_before > 0);

        // Loss-making period.
        engine.record_revenue(50_000).unwrap();
        engine.record_expense(150_000).unwrap();
        let report = engine.close_period().unwrap();

        assert_eq!(report.founder_fee, 0);
        assert_eq!(report.member_distribution, 0);
        assert_eq!(report.reserves_drawn, 100_000);
        assert_eq!(engine.reserves(), reserves_before - 100_000);
    }

    #[test]
    fn test_critical_hub_banks_everything() {
        let mut engine = FiscalEngine::new().unwrap();
        // First period: no reserves yet, so health is Critical and the
        // entire post-fee surplus must rebuild reserves.
        engine.record_revenue(200_000).unwrap();
        engine.record_expense(100_000).unwrap();
        let report = engine.close_period().unwrap();

        assert_eq!(report.member_distribution, 0);
        assert_eq!(report.expansion_deposit, 0);
        let surplus = 100_000u128;
        let fee = surplus / 100;
        assert_eq!(report.founder_fee, fee);
        assert_eq!(report.reserve_deposit, surplus - fee);
    }

    #[test]
    fn test_healthy_hub_distributes() {
        let mut engine = FiscalEngine::new().unwrap();
        // Pump reserves far past target runway.
        engine.record_revenue(10_000_000).unwrap();
        engine.record_expense(100_000).unwrap();
        engine.close_period().unwrap();
        engine.record_revenue(1_000_000).unwrap();
        engine.record_expense(100_000).unwrap();
        let report = engine.close_period().unwrap();

        assert!(report.member_distribution > 0, "healthy hubs pay members");
        assert!(report.expansion_deposit > 0, "healthy hubs fund expansion");
    }

    #[test]
    fn test_conservation_identity() {
        let mut engine = FiscalEngine::new().unwrap();
        for (rev, exp) in [(500_000u128, 200_000u128), (100_000, 300_000), (800_000, 100_000)] {
            engine.record_revenue(rev).unwrap();
            engine.record_expense(exp).unwrap();
            engine.close_period().unwrap();
        }
        assert!(engine.audit().conserved);
    }

    #[test]
    fn test_expansion_requires_track_record() {
        let mut engine = FiscalEngine::new().unwrap();
        engine.record_revenue(100_000_000).unwrap();
        engine.record_expense(100_000).unwrap();
        engine.close_period().unwrap();

        // Rich but unproven: one period is not a track record.
        assert!(engine.try_seed_hub(1_000).is_err());
    }

    #[test]
    fn test_expansion_after_sustained_health() {
        let mut engine = FiscalEngine::new().unwrap();
        for _ in 0..4 {
            engine.record_revenue(10_000_000).unwrap();
            engine.record_expense(100_000).unwrap();
            engine.close_period().unwrap();
        }
        assert_eq!(engine.health(), FiscalHealth::Healthy);
        let fund = engine.expansion_fund();
        assert!(fund > 0);

        let seed = engine.try_seed_hub(fund / 2).unwrap();
        assert_eq!(seed.amount, fund / 2);
        assert!(engine.audit().conserved);
    }
}
