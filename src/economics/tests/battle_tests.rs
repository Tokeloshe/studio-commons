/*!
 * Battle tests for the fiscal engine.
 *
 * Each test is an attack on sustainability, profitability, or conservation.
 * The engine passes only if a hub cannot be bled dry, money cannot be
 * minted or lost, and expansion cannot outrun solvency.
 */

use economics::{
    ConservationAudit, FiscalEngine, FiscalHealth, MIN_RUNWAY_MONTHS,
    SEED_TRACK_RECORD_PERIODS, TARGET_RUNWAY_MONTHS,
};

fn assert_conserved(audit: &ConservationAudit) {
    assert!(
        audit.conserved,
        "conservation violated: revenue {} vs out+held mismatch: {:?}",
        audit.lifetime_revenue, audit
    );
}

// ---------- Sustainability attacks ----------

#[test]
fn attack_bleed_the_hub_dry() {
    // Sustained losses must never pay fees or members, must drain reserves
    // to exactly zero, then flag insolvency — never negative balances.
    let mut engine = FiscalEngine::new().unwrap();

    engine.record_revenue(1_000_000).unwrap();
    engine.record_expense(400_000).unwrap();
    engine.close_period().unwrap();
    let initial_reserves = engine.reserves();
    assert!(initial_reserves > 0);

    let mut periods = 0;
    while engine.health() != FiscalHealth::Insolvent && periods < 100 {
        engine.record_revenue(10_000).unwrap();
        engine.record_expense(200_000).unwrap();
        let report = engine.close_period().unwrap();
        assert_eq!(report.founder_fee, 0, "no fee on losses");
        assert_eq!(report.member_distribution, 0, "no payouts on losses");
        periods += 1;
    }

    assert_eq!(engine.health(), FiscalHealth::Insolvent);
    assert_eq!(engine.reserves(), 0, "reserves drain to exactly zero");
    assert_conserved(&engine.audit());
}

#[test]
fn attack_one_good_month_cannot_unlock_payouts() {
    // A hub with no reserves has one spectacular month. Health was Critical
    // going in, so the surplus must rebuild reserves, not pay out.
    let mut engine = FiscalEngine::new().unwrap();
    engine.record_revenue(5_000_000).unwrap();
    engine.record_expense(1_000_000).unwrap();
    let report = engine.close_period().unwrap();

    assert_eq!(report.member_distribution, 0);
    assert_eq!(report.expansion_deposit, 0);
    assert!(report.reserve_deposit > 0);
}

#[test]
fn attack_zero_activity_period() {
    let mut engine = FiscalEngine::new().unwrap();
    let report = engine.close_period().unwrap();
    assert_eq!(report.surplus, 0);
    assert_eq!(report.deficit, 0);
    assert_eq!(report.founder_fee, 0);
    assert_conserved(&engine.audit());
}

#[test]
fn attack_health_states_follow_runway_exactly() {
    let mut engine = FiscalEngine::new().unwrap();

    // Establish burn of 100_000/month with a barely-surplus period.
    engine.record_revenue(100_001).unwrap();
    engine.record_expense(100_000).unwrap();
    engine.close_period().unwrap();
    assert_eq!(engine.health(), FiscalHealth::Critical);

    // Manufacture reserves via surplus periods and watch states advance.
    let mut reached_rebuilding = false;
    let mut reached_healthy = false;
    for _ in 0..40 {
        engine.record_revenue(200_000).unwrap();
        engine.record_expense(100_000).unwrap();
        engine.close_period().unwrap();
        let runway = engine.reserves() / 100_000;
        match engine.health() {
            FiscalHealth::Critical => assert!(runway < MIN_RUNWAY_MONTHS),
            FiscalHealth::Rebuilding => {
                reached_rebuilding = true;
                assert!(runway >= MIN_RUNWAY_MONTHS && runway < TARGET_RUNWAY_MONTHS);
            }
            FiscalHealth::Healthy => {
                reached_healthy = true;
                assert!(runway >= TARGET_RUNWAY_MONTHS);
            }
            FiscalHealth::Insolvent => panic!("surplus periods cannot cause insolvency"),
        }
    }
    assert!(reached_rebuilding && reached_healthy, "must climb through all states");
}

#[test]
fn attack_insolvent_hub_must_repay_debt_before_any_profit() {
    let mut engine = FiscalEngine::new().unwrap();

    // Straight into insolvency: bills with no money.
    engine.record_expense(500_000).unwrap();
    let report = engine.close_period().unwrap();
    assert_eq!(report.unfunded_deficit, 500_000);
    assert_eq!(engine.health(), FiscalHealth::Insolvent);

    // A boom arrives. Debt must be settled before a single unit is fee,
    // payout, or reserve.
    engine.record_revenue(600_000).unwrap();
    engine.record_expense(50_000).unwrap();
    let report = engine.close_period().unwrap();

    assert_eq!(report.debt_repaid, 500_000);
    assert_eq!(report.surplus, 50_000, "profit is what remains after debts");
    assert_eq!(report.founder_fee, 500, "fee applies only to post-debt surplus");
    assert_ne!(engine.health(), FiscalHealth::Insolvent, "debt cleared, hub recovers");
    assert_conserved(&engine.audit());
}

// ---------- Profitability attacks ----------

#[test]
fn attack_fee_only_on_true_profit() {
    // Fee must be exactly 1% of (revenue - expenses), never of revenue.
    let mut engine = FiscalEngine::new().unwrap();
    engine.record_revenue(1_000_000).unwrap();
    engine.record_expense(900_000).unwrap();
    let report = engine.close_period().unwrap();

    assert_eq!(report.surplus, 100_000);
    assert_eq!(report.founder_fee, 1_000, "1% of surplus, not of revenue");
}

#[test]
fn attack_waterfall_is_exact_no_dust_lost() {
    // In every state, the full surplus must be exactly partitioned:
    // fee + members + reinvest + expansion + reserves == surplus.
    let mut engine = FiscalEngine::new().unwrap();

    // Awkward primes to force rounding at every division.
    let cases: Vec<(u128, u128)> = vec![
        (1_000_003, 999_983),
        (7_777_777, 3_333_331),
        (104_729, 104_723),
        (999_999_999_989, 17),
    ];
    for (rev, exp) in cases {
        engine.record_revenue(rev).unwrap();
        engine.record_expense(exp).unwrap();
        let r = engine.close_period().unwrap();
        if r.surplus > 0 {
            let allocated = r.founder_fee
                + r.member_distribution
                + r.local_reinvestment
                + r.expansion_deposit
                + r.reserve_deposit;
            assert_eq!(allocated, r.surplus, "dust lost at ({}, {})", rev, exp);
        }
    }
    assert_conserved(&engine.audit());
}

#[test]
fn attack_extreme_values_do_not_break_math() {
    // A ludicrous economy: revenue near practical limits. Integer math must
    // neither overflow nor leak.
    let mut engine = FiscalEngine::new().unwrap();
    let huge = u128::MAX / 20_000; // safe headroom under the bps multiplier
    engine.record_revenue(huge).unwrap();
    engine.record_expense(huge / 2).unwrap();
    let report = engine.close_period().unwrap();

    assert_eq!(report.surplus, huge - huge / 2);
    let allocated = report.founder_fee
        + report.member_distribution
        + report.local_reinvestment
        + report.expansion_deposit
        + report.reserve_deposit;
    assert_eq!(allocated, report.surplus);
    assert_conserved(&engine.audit());
}

// ---------- Expansion attacks ----------

#[test]
fn attack_cannot_seed_while_critical() {
    let mut engine = FiscalEngine::new().unwrap();
    engine.record_revenue(150_000).unwrap();
    engine.record_expense(100_000).unwrap();
    engine.close_period().unwrap();
    assert_ne!(engine.health(), FiscalHealth::Healthy);
    assert!(engine.try_seed_hub(1).is_err());
}

#[test]
fn attack_cannot_seed_beyond_expansion_fund() {
    let mut engine = FiscalEngine::new().unwrap();
    for _ in 0..SEED_TRACK_RECORD_PERIODS + 1 {
        engine.record_revenue(10_000_000).unwrap();
        engine.record_expense(100_000).unwrap();
        engine.close_period().unwrap();
    }
    assert_eq!(engine.health(), FiscalHealth::Healthy);

    let fund = engine.expansion_fund();
    assert!(fund > 0);
    // One unit more than the fund: rejected, reserves untouched.
    let reserves_before = engine.reserves();
    assert!(engine.try_seed_hub(fund + 1).is_err());
    assert_eq!(engine.reserves(), reserves_before, "reserves must never fund seeds");
    assert_eq!(engine.expansion_fund(), fund);
}

#[test]
fn attack_flash_prosperity_cannot_trigger_expansion() {
    // Profitable-and-healthy for 2 periods, then a loss, then profitable
    // again: the loss must reset eligibility until a full fresh track record.
    let mut engine = FiscalEngine::new().unwrap();
    for _ in 0..2 {
        engine.record_revenue(10_000_000).unwrap();
        engine.record_expense(100_000).unwrap();
        engine.close_period().unwrap();
    }
    // Loss period.
    engine.record_revenue(0).unwrap();
    engine.record_expense(100_000).unwrap();
    engine.close_period().unwrap();
    // One more good period — track record now [good, loss, good].
    engine.record_revenue(10_000_000).unwrap();
    engine.record_expense(100_000).unwrap();
    engine.close_period().unwrap();

    assert!(
        engine.try_seed_hub(1).is_err(),
        "a loss inside the window must block seeding"
    );

    // Two more clean periods complete a fresh 3-period record.
    for _ in 0..2 {
        engine.record_revenue(10_000_000).unwrap();
        engine.record_expense(100_000).unwrap();
        engine.close_period().unwrap();
    }
    assert!(engine.try_seed_hub(1).is_ok());
    assert_conserved(&engine.audit());
}

#[test]
fn attack_zero_cost_seed_rejected() {
    let mut engine = FiscalEngine::new().unwrap();
    assert!(engine.try_seed_hub(0).is_err());
}

// ---------- Long-run stress fuzz ----------

/// Deterministic LCG so the fuzz is reproducible (no clock, no OS rng).
struct Lcg(u128);
impl Lcg {
    fn next(&mut self) -> u128 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

#[test]
fn attack_five_hundred_period_economy_holds_every_invariant() {
    let mut engine = FiscalEngine::new().unwrap();
    let mut rng = Lcg(0x5EED_C0FFEE);
    let mut seeded_hubs = 0u32;

    for period in 0..500 {
        // Revenue and expenses swing wildly: booms, busts, dead months.
        let revenue = rng.next() % 2_000_000;
        let expenses = rng.next() % 1_500_000;
        engine.record_revenue(revenue).unwrap();
        engine.record_expense(expenses).unwrap();
        let report = engine.close_period().unwrap();

        // Invariant 1: money is conserved after every single period.
        assert_conserved(&engine.audit());

        // Invariant 2: surplus is exactly partitioned.
        if report.surplus > 0 {
            assert_eq!(
                report.founder_fee
                    + report.member_distribution
                    + report.local_reinvestment
                    + report.expansion_deposit
                    + report.reserve_deposit,
                report.surplus,
                "period {} leaked",
                period
            );
        }

        // Invariant 3: losses never pay anyone.
        if report.deficit > 0 {
            assert_eq!(report.founder_fee + report.member_distribution, 0);
        }

        // Invariant 4: an insolvent-flagged engine has zero reserves.
        if engine.health() == FiscalHealth::Insolvent {
            assert_eq!(engine.reserves(), 0);
        }

        // Opportunistic expansion whenever the engine allows it.
        let fund = engine.expansion_fund();
        if fund > 100_000 {
            if engine.try_seed_hub(fund / 2).is_ok() {
                seeded_hubs += 1;
                assert_conserved(&engine.audit());
            }
        }
    }

    let audit = engine.audit();
    assert_conserved(&audit);
    assert!(
        seeded_hubs > 0,
        "a 500-period economy with booms should expand at least once"
    );
    assert!(
        audit.lifetime_member_distributions > 0,
        "members must get paid across a long mixed economy"
    );
}
