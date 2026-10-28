// SPDX-License-Identifier: AGPL-3.0-or-later
//! The scenario behind docs/media/studio-commons.svg.
//!
//! Runs the real crates (cci, economics, network) through one story and
//! prints every number the animation shows, as JSON:
//!
//!   cargo run --release --example film > /tmp/film.json
//!
//! Nothing here re-implements a rule. Merit, the waterfall, health states,
//! seeding and the audits all come from the library calls below.

use cci::{CCISystem, ContributionType, PeerReview};
use chrono::Utc;
use economics::FiscalHealth;
use network::HubNetwork;
use serde_json::{json, Value};
use uuid::Uuid;

/// A licence fee the film earns ten years on, paid through the same ledger.
const LATER_LICENCE: u128 = 12_000;

fn id(n: u128) -> Uuid {
    Uuid::from_u128(0x5c00_0000_0000_0000_0000_0000_0000_0000 + n)
}

fn review(who: Uuid, score: f64) -> PeerReview {
    PeerReview { reviewer: who, impact_score: score, timestamp: Utc::now() }
}

fn health(h: FiscalHealth) -> &'static str {
    match h {
        FiscalHealth::Insolvent => "insolvent",
        FiscalHealth::Critical => "critical",
        FiscalHealth::Rebuilding => "rebuilding",
        FiscalHealth::Healthy => "healthy",
    }
}

fn main() -> anyhow::Result<()> {
    // ------------------------------------------------------------ merit
    let mut cci = CCISystem::new()?;
    let film = id(900);
    let crew = [
        ("Ava", "Direction", ContributionType::Direction, 60.0, vec![0.88, 0.82, 0.91]),
        ("Ben", "Editing", ContributionType::Editing, 72.0, vec![0.74, 0.80, 0.77]),
        ("Cleo", "Sound", ContributionType::Sound, 30.0, vec![0.90, 0.62, 0.85]),
        ("Dev", "Camera", ContributionType::Cinematography, 48.0, vec![0.81, 0.79, 0.84]),
        ("Eli", "Production", ContributionType::Production, 40.0, vec![0.62, 0.66, 0.60, 0.69, 1.0]),
        ("Fay", "Set build", ContributionType::Design, 16.0, vec![0.93, 0.86, 0.90]),
    ];
    let reviewers: Vec<Uuid> = (0..5).map(|i| id(500 + i)).collect();
    let mut entries = vec![];
    let mut ids = vec![];
    for (i, (name, role, kind, hours, scores)) in crew.iter().enumerate() {
        let member = id(100 + i as u128);
        let rs: Vec<PeerReview> =
            scores.iter().enumerate().map(|(k, s)| review(reviewers[k], *s)).collect();
        let cid = cci.add_contribution(member, film, kind.clone(), *hours, rs)?;
        ids.push(cid);
        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        entries.push(json!({
            "name": name, "role": role, "hours": hours, "scores": scores,
            "mean": mean, "member": member.to_string(),
        }));
    }
    // Attacks the ledger refuses, with the library's own words.
    let refusals = vec![
        ("self-review", cci
            .add_review(ids[1], review(id(101), 1.0))
            .err().map(|e| e.to_string()).unwrap_or_default()),
        ("conflict of interest", cci
            .add_review(ids[0], review(id(102), 1.0))
            .err().map(|e| e.to_string()).unwrap_or_default()),
        ("hour cap", cci
            .add_contribution(id(103), film, ContributionType::Sound, 120.0,
                (0..3).map(|k| review(reviewers[k], 0.9)).collect())
            .err().map(|e| e.to_string()).unwrap_or_default()),
    ];
    let scores = cci.compute_scores();
    let mut merit = vec![];
    for (i, e) in entries.iter_mut().enumerate() {
        let p = scores.get(&id(100 + i as u128)).map(|s| s.total_points).unwrap_or(0.0);
        e["points"] = json!(p);
        e["median"] = json!(p / e["hours"].as_f64().unwrap());
        merit.push(p);
    }
    let residual = |amount: u128, year: u32| -> anyhow::Result<Value> {
        let shares = cci.global_residuals(amount, film, year)?;
        Ok(json!(shares.iter().map(|s| json!({
            "member": s.member_id.to_string(), "pct": s.share_percentage, "amount": s.amount
        })).collect::<Vec<_>>()))
    };
    let ledger_audit = cci.audit();

    // ------------------------------------------------------------ money
    let mut net = HubNetwork::new();
    let la = net.found_hub("Los Angeles")?;
    // (revenue, expenses) per month for the LA hub
    let la_months: Vec<(u128, u128)> = vec![
        (71_000, 42_000), (76_000, 42_000), (74_500, 43_000), (79_000, 42_500),
        (82_000, 43_500), (80_500, 44_000), (83_000, 44_000), (78_000, 45_000),
        (29_000, 47_500), (84_000, 44_500), (88_000, 45_000), (91_000, 45_500),
        (95_000, 46_000), (168_000, 47_000), (99_000, 46_500), (104_000, 47_000),
        (102_000, 47_500), (108_000, 48_000), (111_000, 48_000), (106_000, 48_500),
        (114_000, 49_000), (117_000, 49_000), (112_000, 49_500), (121_000, 50_000),
    ];
    let seed_cost: u128 = 60_000;
    let mut months = vec![];
    let mut seeds = vec![];
    let mut hubs = vec![("Los Angeles", la, None::<usize>)];
    let city_plan = ["New York", "Berlin", "Mumbai", "Lagos", "São Paulo"];
    let mut next_city = 0;
    // other hubs' trajectories: modest, steady
    let child_months = |k: usize, gen: usize| -> (u128, u128) {
        let base = 52_000 + 1_800 * k as u128 + 4_000 * gen as u128;
        (base + if k % 7 == 3 { 0 } else { 14_000 + 900 * k as u128 }, 38_000 + 400 * k as u128)
    };
    let total_months = 48;
    let mut child_age: Vec<usize> = vec![0];
    for m in 0..total_months {
        for (hi, (_, hid, _)) in hubs.clone().iter().enumerate() {
            let (rev, exp) = if hi == 0 {
                la_months.get(m).copied().unwrap_or((118_000 + 900 * (m as u128 % 5), 50_000))
            } else {
                child_months(child_age[hi], 1)
            };
            net.record_revenue(*hid, rev)?;
            net.record_expense(*hid, exp)?;
            let pre = net.hub(*hid)?.engine.health();
            let r = net.close_period(*hid)?;
            let eng = &net.hub(*hid)?.engine;
            if hi == 0 {
                months.push(json!({
                    "m": m, "revenue": r.revenue, "expenses": r.expenses,
                    "surplus": r.surplus, "deficit": r.deficit,
                    "reserves_drawn": r.reserves_drawn, "fee": r.founder_fee,
                    "members": r.member_distribution, "reinvest": r.local_reinvestment,
                    "expansion": r.expansion_deposit, "reserve_deposit": r.reserve_deposit,
                    "reserves": r.reserves_after, "fund": r.expansion_fund_after,
                    "pre": health(pre), "after": health(r.health_after),
                    "runway": eng.runway_months().min(99) as u64,
                }));
            }
            child_age[hi] += 1;
        }
        // grow whenever the strongest qualified hub can afford a seed
        while next_city < city_plan.len() {
            match net.expand(city_plan[next_city], seed_cost) {
                Ok((child, sponsor)) => {
                    let si = hubs.iter().position(|h| h.1 == sponsor).unwrap();
                    seeds.push(json!({ "month": m, "city": city_plan[next_city],
                                       "sponsor": hubs[si].0, "amount": seed_cost }));
                    hubs.push((city_plan[next_city], child, Some(si)));
                    child_age.push(0);
                    next_city += 1;
                }
                Err(_) => break,
            }
        }
    }
    let la_audit = net.hub(la)?.engine.audit();
    let net_audit = net.audit();
    let rep = net.report();

    // the first month paid out in full (Healthy before it closed)
    let paid = months.iter().find(|m| m["pre"] == "healthy" && m["members"].as_u64().unwrap_or(0) > 0).cloned();
    let pool = paid.as_ref().map(|m| m["members"].as_u64().unwrap() as u128).unwrap_or(0);

    let out = json!({
        "crew": entries,
        "refusals": refusals.iter().map(|(k, v)| json!({"kind": k, "error": v})).collect::<Vec<_>>(),
        "ledger": { "entries": ledger_audit.entries, "chain_valid": ledger_audit.chain_valid,
                    "head": format!("{:016X}", cci.head_hash()) },
        "residuals": { "year1": residual(pool, 1)?, "year10": residual(LATER_LICENCE, 10)?,
                        "later": LATER_LICENCE,
                        "pool_month": paid.as_ref().map(|m| m["m"].clone()), "pool": pool },
        "months": months,
        "seed_cost": seed_cost,
        "seeds": seeds,
        "hubs": hubs.iter().map(|(n, _, p)| json!({"name": n, "parent": p})).collect::<Vec<_>>(),
        "audit": {
            "lifetime_revenue": la_audit.lifetime_revenue,
            "expenses_paid": la_audit.lifetime_expenses_paid,
            "fees": la_audit.lifetime_founder_fees,
            "members": la_audit.lifetime_member_distributions,
            "reinvest": la_audit.lifetime_reinvestment,
            "seeds": la_audit.lifetime_hub_seeds,
            "reserves": la_audit.reserves_held,
            "fund": la_audit.expansion_fund_held,
            "conserved": la_audit.conserved,
            "network_conserved": net_audit.conserved,
            "hubs": rep.hubs, "generations": rep.generations,
        },
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
