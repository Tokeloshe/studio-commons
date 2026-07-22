/*!
 * Battle tests for the hub network.
 *
 * Attacks target the firewall between hubs, sponsor selection, lineage
 * integrity, and network-wide conservation across generations of growth.
 */

use economics::FiscalHealth;
use network::HubNetwork;
use uuid::Uuid;

fn run_periods(net: &mut HubNetwork, hub: Uuid, n: usize, revenue: u128, expense: u128) {
    for _ in 0..n {
        net.record_revenue(hub, revenue).unwrap();
        net.record_expense(hub, expense).unwrap();
        net.close_period(hub).unwrap();
    }
}

// ---------- Firewall attacks ----------

#[test]
fn attack_child_insolvency_cannot_touch_parent() {
    let mut net = HubNetwork::new();
    let parent = net.found_hub("LA").unwrap();
    run_periods(&mut net, parent, 4, 10_000_000, 100_000);

    let fund = net.hub(parent).unwrap().engine.expansion_fund();
    let (child, _) = net.expand("NYC", fund / 2).unwrap();

    let parent_reserves = net.hub(parent).unwrap().engine.reserves();

    // Child burns straight into insolvency.
    for _ in 0..20 {
        net.record_expense(child, 1_000_000).unwrap();
        net.close_period(child).unwrap();
    }
    assert_eq!(
        net.hub(child).unwrap().engine.health(),
        FiscalHealth::Insolvent
    );

    // Parent is untouched: its reserves did not move one unit.
    assert_eq!(net.hub(parent).unwrap().engine.reserves(), parent_reserves);
    // And each hub's books still conserve independently.
    assert!(net.audit().all_hubs_conserved);
}

#[test]
fn attack_weak_hub_cannot_sponsor() {
    let mut net = HubNetwork::new();
    let strong = net.found_hub("LA").unwrap();
    let weak = net.found_hub("Fresno").unwrap();

    run_periods(&mut net, strong, 4, 10_000_000, 100_000);
    // Weak hub: barely breaking even, never Healthy.
    run_periods(&mut net, weak, 4, 100_001, 100_000);

    let rankings = net.sponsor_rankings();
    assert_eq!(rankings.len(), 1, "only the strong hub qualifies");
    assert_eq!(rankings[0].hub_id, strong);

    let (_, sponsor) = net.expand("NYC", 1_000).unwrap();
    assert_eq!(sponsor, strong, "expansion must come from the strong hub");
}

// ---------- Sponsor-selection attacks ----------

#[test]
fn attack_sponsor_choice_is_strongest_and_deterministic() {
    let mut net = HubNetwork::new();
    let a = net.found_hub("A").unwrap();
    let b = net.found_hub("B").unwrap();

    // Both Healthy with track records, but B has the longer streak.
    run_periods(&mut net, a, 4, 10_000_000, 100_000);
    run_periods(&mut net, b, 8, 10_000_000, 100_000);

    for _ in 0..3 {
        let rankings = net.sponsor_rankings();
        assert_eq!(rankings[0].hub_id, b, "longest healthy streak leads");
        assert_eq!(rankings[1].hub_id, a);
    }

    let (_, sponsor) = net.expand("C", 1_000).unwrap();
    assert_eq!(sponsor, b);
}

#[test]
fn attack_one_loss_knocks_hub_out_of_rankings() {
    let mut net = HubNetwork::new();
    let hub = net.found_hub("LA").unwrap();
    run_periods(&mut net, hub, 4, 10_000_000, 100_000);
    assert_eq!(net.sponsor_rankings().len(), 1);

    // A single loss period resets the streak below the required record.
    net.record_expense(hub, 100_000).unwrap();
    net.close_period(hub).unwrap();
    assert!(net.sponsor_rankings().is_empty());
    assert!(net.expand("NYC", 1).is_err());
}

#[test]
fn attack_unaffordable_seed_rejected_even_when_qualified() {
    let mut net = HubNetwork::new();
    let hub = net.found_hub("LA").unwrap();
    run_periods(&mut net, hub, 4, 10_000_000, 100_000);

    let fund = net.hub(hub).unwrap().engine.expansion_fund();
    assert!(net.expand("NYC", fund + 1).is_err());
    // Failed expansion must leave no phantom hub behind.
    assert_eq!(net.report().hubs, 1);
    assert!(net.audit().conserved);
}

// ---------- Lineage attacks ----------

#[test]
fn attack_lineage_is_acyclic_and_generations_count_up() {
    let mut net = HubNetwork::new();
    let root = net.found_hub("gen0").unwrap();
    run_periods(&mut net, root, 4, 10_000_000, 100_000);

    // Grow a 3-generation chain: each child earns its own track record,
    // then sponsors the next generation.
    let mut latest = root;
    for gen in 1..=3u32 {
        let fund = net.hub(latest).unwrap().engine.expansion_fund();
        let (child, sponsor) = net.expand(&format!("gen{}", gen), fund / 2).unwrap();
        assert_eq!(sponsor, latest, "the streak leader should be the sponsor");
        assert_eq!(net.hub(child).unwrap().generation, gen);

        let lineage = net.lineage(child).unwrap();
        assert_eq!(lineage.len() as u32, gen + 1, "lineage length = generation + 1");
        assert_eq!(*lineage.last().unwrap(), root, "all roads lead to the root");
        // Acyclic: no hub repeats in its own ancestry.
        let mut seen = lineage.clone();
        seen.sort();
        seen.dedup();
        assert_eq!(seen.len(), lineage.len());

        // The child runs a stronger economy than its ancestors, earning the
        // longest streak so it sponsors the next generation.
        run_periods(&mut net, child, 10 + gen as usize, 20_000_000, 100_000);
        latest = child;
    }

    assert_eq!(net.report().generations, 3);
    assert!(net.audit().conserved);
}

// ---------- Long-run network fuzz ----------

struct Lcg(u128);
impl Lcg {
    fn next(&mut self) -> u128 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

#[test]
fn attack_multi_generation_economy_conserves_everything() {
    let mut net = HubNetwork::new();
    let mut rng = Lcg(0x0EED_5EED_2026);

    let root = net.found_hub("root").unwrap();
    let mut expansions = 0u32;

    for round in 0..120 {
        // Every hub trades each round with wildly varying fortunes.
        let ids: Vec<Uuid> = net.hub_ids().to_vec();
        for id in &ids {
            let revenue = rng.next() % 5_000_000;
            let expense = rng.next() % 3_000_000;
            net.record_revenue(*id, revenue).unwrap();
            net.record_expense(*id, expense).unwrap();
            net.close_period(*id).unwrap();
        }

        // The network expands whenever any hub has earned the right.
        if net.report().hubs < 12 {
            let best_fund = net
                .sponsor_rankings()
                .first()
                .map(|s| s.expansion_fund)
                .unwrap_or(0);
            if best_fund > 200_000 {
                if net.expand(&format!("hub-{}", round), best_fund / 2).is_ok() {
                    expansions += 1;
                }
            }
        }

        // Invariants after every round:
        let audit = net.audit();
        assert!(audit.all_hubs_conserved, "hub books broke at round {}", round);
        assert!(
            audit.seeds_match_capital,
            "seed money leaked at round {}: out {} vs in {}",
            round, audit.total_seed_outflow, audit.total_seed_capital
        );

        // Lineage stays sound for every hub.
        for id in net.hub_ids().to_vec() {
            let lineage = net.lineage(id).unwrap();
            assert_eq!(*lineage.last().unwrap(), root);
            assert_eq!(
                net.hub(id).unwrap().generation as usize,
                lineage.len() - 1
            );
        }
    }

    assert!(expansions > 0, "a 120-round economy with booms should expand");
    let report = net.report();
    assert_eq!(report.hubs as u32, expansions + 1);
}
