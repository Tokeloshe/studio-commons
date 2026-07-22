/*!
 * Network Module - coordinates a fleet of hubs into one expanding network.
 *
 * Each hub runs its own FiscalEngine; the network layer decides *where*
 * growth happens. Principles:
 *
 * - Expansion flows from strength. When the network expands, the sponsor
 *   is the hub with the longest current streak of profitable, Healthy
 *   periods — growth is funded where the model is proving strongest, by a
 *   deterministic ranking every member can recompute.
 * - Hubs are firewalled. A hub's obligations are its own: one hub's
 *   insolvency cannot draw down another hub's reserves. The network can
 *   only move money at one moment — seeding — and only from an expansion
 *   fund that exists because of sustained surplus.
 * - Nothing crosses between hubs unaccounted. Every seed leaving a
 *   sponsor arrives as seed capital in exactly one child, and the network
 *   audit proves the two totals match while every hub's own books conserve.
 * - Lineage is permanent. Every hub records who seeded it; the family tree
 *   is acyclic by construction because a parent must exist before its child.
 */

use anyhow::{bail, Result};
use economics::{FiscalEngine, FiscalHealth, PeriodReport};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A hub in the network: a fiscal engine plus its place in the family tree.
pub struct NetworkHub {
    pub id: Uuid,
    pub name: String,
    pub parent: Option<Uuid>,
    /// Root hubs are generation 0; a child is its parent's generation + 1.
    pub generation: u32,
    /// Capital received at founding from the sponsor's expansion fund.
    pub seed_capital: u128,
    pub engine: FiscalEngine,
}

/// Why a hub qualifies (or how strongly) as an expansion sponsor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SponsorScore {
    pub hub_id: Uuid,
    /// Current streak of consecutive closed periods that were both
    /// profitable and Healthy. The primary measure of proven strength.
    pub healthy_streak: usize,
    pub expansion_fund: u128,
}

/// Aggregate state of the whole network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkReport {
    pub hubs: usize,
    pub generations: u32,
    pub healthy: usize,
    pub rebuilding: usize,
    pub critical: usize,
    pub insolvent: usize,
    pub total_reserves: u128,
    pub total_expansion_funds: u128,
}

/// Network-wide conservation audit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAudit {
    /// Every hub's own books conserve exactly.
    pub all_hubs_conserved: bool,
    /// Every unit that left a sponsor as a seed arrived as seed capital
    /// in exactly one child.
    pub seeds_match_capital: bool,
    pub total_seed_outflow: u128,
    pub total_seed_capital: u128,
    pub conserved: bool,
}

pub struct HubNetwork {
    hubs: HashMap<Uuid, NetworkHub>,
    /// Founding order, for deterministic iteration.
    order: Vec<Uuid>,
}

/// Current streak of consecutive trailing periods that were profitable and
/// Healthy — the network's measure of a proven hub.
fn healthy_streak(reports: &[PeriodReport]) -> usize {
    reports
        .iter()
        .rev()
        .take_while(|r| r.surplus > 0 && r.health_after == FiscalHealth::Healthy)
        .count()
}

impl HubNetwork {
    pub fn new() -> Self {
        info!("Initializing Hub Network");
        Self {
            hubs: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// Found an independent root hub (generation 0, no sponsor). Root hubs
    /// bootstrap from their own community, not from network funds.
    pub fn found_hub(&mut self, name: &str) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let hub = NetworkHub {
            id,
            name: name.to_string(),
            parent: None,
            generation: 0,
            seed_capital: 0,
            engine: FiscalEngine::new()?,
        };
        info!("Founded root hub '{}' ({})", name, id);
        self.hubs.insert(id, hub);
        self.order.push(id);
        Ok(id)
    }

    pub fn record_revenue(&mut self, hub_id: Uuid, amount: u128) -> Result<()> {
        self.hub_mut(hub_id)?.engine.record_revenue(amount)
    }

    pub fn record_expense(&mut self, hub_id: Uuid, amount: u128) -> Result<()> {
        self.hub_mut(hub_id)?.engine.record_expense(amount)
    }

    pub fn close_period(&mut self, hub_id: Uuid) -> Result<PeriodReport> {
        self.hub_mut(hub_id)?.engine.close_period()
    }

    /// Rank every hub currently eligible to sponsor expansion, strongest
    /// first. Eligibility mirrors FiscalEngine::try_seed_hub (Healthy now,
    /// with a full profitable-and-Healthy track record); ordering is by
    /// longest healthy streak, then largest expansion fund, then hub id —
    /// fully deterministic, so any member can recompute the ranking.
    pub fn sponsor_rankings(&self) -> Vec<SponsorScore> {
        let mut ranked: Vec<SponsorScore> = self
            .order
            .iter()
            .filter_map(|id| self.hubs.get(id))
            .filter(|hub| {
                hub.engine.health() == FiscalHealth::Healthy
                    && healthy_streak(hub.engine.reports())
                        >= economics::SEED_TRACK_RECORD_PERIODS
            })
            .map(|hub| SponsorScore {
                hub_id: hub.id,
                healthy_streak: healthy_streak(hub.engine.reports()),
                expansion_fund: hub.engine.expansion_fund(),
            })
            .collect();

        ranked.sort_by(|a, b| {
            b.healthy_streak
                .cmp(&a.healthy_streak)
                .then(b.expansion_fund.cmp(&a.expansion_fund))
                .then(a.hub_id.cmp(&b.hub_id))
        });
        ranked
    }

    /// Expand the network: seed a new hub from the strongest eligible
    /// sponsor that can afford it. The seed leaves the sponsor's expansion
    /// fund (its engine enforces every solvency rule) and arrives as the
    /// child's founding capital.
    pub fn expand(&mut self, name: &str, seed_cost: u128) -> Result<(Uuid, Uuid)> {
        let rankings = self.sponsor_rankings();
        if rankings.is_empty() {
            bail!("no hub currently qualifies to sponsor expansion");
        }

        let sponsor_id = rankings
            .iter()
            .find(|s| s.expansion_fund >= seed_cost)
            .map(|s| s.hub_id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "no qualified hub can afford seed cost {} (best fund: {})",
                    seed_cost,
                    rankings.iter().map(|s| s.expansion_fund).max().unwrap_or(0)
                )
            })?;

        let (sponsor_generation, seed) = {
            let sponsor = self.hub_mut(sponsor_id)?;
            let seed = sponsor.engine.try_seed_hub(seed_cost)?;
            (sponsor.generation, seed)
        };

        let child_id = Uuid::new_v4();
        let mut engine = FiscalEngine::new()?;
        // Seed capital enters the child's books as its founding revenue so
        // the child's own conservation audit accounts for it; the network
        // audit then proves sponsor outflows equal child capital exactly.
        engine.record_revenue(seed.amount)?;

        let child = NetworkHub {
            id: child_id,
            name: name.to_string(),
            parent: Some(sponsor_id),
            generation: sponsor_generation + 1,
            seed_capital: seed.amount,
            engine,
        };
        info!(
            "Expanded: '{}' ({}) seeded with {} by sponsor {}",
            name, child_id, seed.amount, sponsor_id
        );
        self.hubs.insert(child_id, child);
        self.order.push(child_id);

        Ok((child_id, sponsor_id))
    }

    /// Path from a hub to its founding root, child first.
    pub fn lineage(&self, hub_id: Uuid) -> Result<Vec<Uuid>> {
        let mut path = Vec::new();
        let mut current = Some(hub_id);
        while let Some(id) = current {
            let hub = self
                .hubs
                .get(&id)
                .ok_or_else(|| anyhow::anyhow!("unknown hub {}", id))?;
            path.push(id);
            current = hub.parent;
        }
        Ok(path)
    }

    /// Aggregate report across the network.
    pub fn report(&self) -> NetworkReport {
        let mut report = NetworkReport {
            hubs: self.hubs.len(),
            generations: 0,
            healthy: 0,
            rebuilding: 0,
            critical: 0,
            insolvent: 0,
            total_reserves: 0,
            total_expansion_funds: 0,
        };
        for hub in self.hubs.values() {
            report.generations = report.generations.max(hub.generation);
            match hub.engine.health() {
                FiscalHealth::Healthy => report.healthy += 1,
                FiscalHealth::Rebuilding => report.rebuilding += 1,
                FiscalHealth::Critical => report.critical += 1,
                FiscalHealth::Insolvent => report.insolvent += 1,
            }
            report.total_reserves += hub.engine.reserves();
            report.total_expansion_funds += hub.engine.expansion_fund();
        }
        report
    }

    /// Prove the network's books: every hub conserves internally, and every
    /// unit seeded out of sponsors equals the seed capital received by
    /// children.
    pub fn audit(&self) -> NetworkAudit {
        let all_hubs_conserved = self.hubs.values().all(|h| h.engine.audit().conserved);

        let total_seed_outflow: u128 = self
            .hubs
            .values()
            .flat_map(|h| h.engine.seeds())
            .map(|s| s.amount)
            .sum();
        let total_seed_capital: u128 = self.hubs.values().map(|h| h.seed_capital).sum();
        let seeds_match_capital = total_seed_outflow == total_seed_capital;

        NetworkAudit {
            all_hubs_conserved,
            seeds_match_capital,
            total_seed_outflow,
            total_seed_capital,
            conserved: all_hubs_conserved && seeds_match_capital,
        }
    }

    pub fn hub(&self, hub_id: Uuid) -> Result<&NetworkHub> {
        self.hubs
            .get(&hub_id)
            .ok_or_else(|| anyhow::anyhow!("unknown hub {}", hub_id))
    }

    fn hub_mut(&mut self, hub_id: Uuid) -> Result<&mut NetworkHub> {
        self.hubs
            .get_mut(&hub_id)
            .ok_or_else(|| anyhow::anyhow!("unknown hub {}", hub_id))
    }

    /// Hub ids in founding order.
    pub fn hub_ids(&self) -> &[Uuid] {
        &self.order
    }
}

impl Default for HubNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_healthy_periods(net: &mut HubNetwork, hub: Uuid, n: usize) {
        for _ in 0..n {
            net.record_revenue(hub, 10_000_000).unwrap();
            net.record_expense(hub, 100_000).unwrap();
            net.close_period(hub).unwrap();
        }
    }

    #[test]
    fn test_found_and_report() {
        let mut net = HubNetwork::new();
        let la = net.found_hub("LA").unwrap();
        assert_eq!(net.report().hubs, 1);
        assert_eq!(net.hub(la).unwrap().generation, 0);
    }

    #[test]
    fn test_expansion_creates_child_with_capital() {
        let mut net = HubNetwork::new();
        let la = net.found_hub("LA").unwrap();
        run_healthy_periods(&mut net, la, 4);

        let fund = net.hub(la).unwrap().engine.expansion_fund();
        assert!(fund > 0);

        let (child, sponsor) = net.expand("NYC", fund / 2).unwrap();
        assert_eq!(sponsor, la);
        let child_hub = net.hub(child).unwrap();
        assert_eq!(child_hub.generation, 1);
        assert_eq!(child_hub.seed_capital, fund / 2);
        assert_eq!(net.lineage(child).unwrap(), vec![child, la]);
        assert!(net.audit().conserved);
    }

    #[test]
    fn test_no_sponsor_no_expansion() {
        let mut net = HubNetwork::new();
        net.found_hub("LA").unwrap();
        assert!(net.expand("NYC", 1).is_err());
    }
}
