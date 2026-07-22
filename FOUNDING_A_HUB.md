# Founding a Studio Commons Hub

**A complete guide to starting a community-owned studio in your city — no permission required.**

This document exists so that you can do this *without us*. Studio Commons is AGPL-licensed infrastructure with deterministic rules: there is no head office to apply to, no franchise fee, no founder who can veto your hub. If you can gather a community and follow this playbook, you can run one. That is not a loophole — it is the design.

## The journey at a glance

| Phase | What happens | Time | Cost | You're done when |
|---|---|---|---|---|
| **[0 — Prove demand](#phase-0--prove-the-demand-weeks-14-cost-0)** | Founding circle, survey, written commitments | 4 weeks | $0 | Committed dues ≥ 1.5× projected costs |
| **[1 — Run the software](#phase-1--run-the-software-before-the-studio-exists-week-2-onward)** | Install, dry-run your real numbers | days | $0 | You've closed a paper period and read the report |
| **[2 — Incorporate](#phase-2--incorporate-and-adopt-the-constitution-weeks-48)** | Entity, bylaws that bind the code, bank account | 4 weeks | filing fees | Bylaws adopted, account open |
| **[3 — Open](#phase-3--open-month-24)** | Minimum viable offering, live books, CCI ledger | 2 months | your dues | First real period closed and published |
| **[4 — Reach Healthy](#phase-4--reach-healthy-months-418)** | Build 6 months runway, 3 healthy periods | ~1 year | — | Full distributions + sponsor eligibility unlock |
| **[5 — Seed the next city](#phase-5--seed-the-next-city-year-2)** | Your surplus births the next hub | year 2+ | surplus only | Another city runs because yours worked |

**⚡ Shortest possible first step, right now:** `git clone` this repo and run `cargo test --all`. Zero cost, no signup — and you'll have personally verified every economic rule in this guide before trusting any of it.

---

## Who this is for

You, if some of these are true:

- You're a filmmaker, musician, photographer, or producer tired of renting overpriced infrastructure from people who extract everything and credit no one.
- You have (or can find) 20–100 creators in your city who feel the same.
- Someone in your group can run software and read a spreadsheet. You don't need a blockchain engineer. You need one careful person.

## What you're actually building

A **hub** is three things bound together:

1. **A physical or virtual creative space** — a stage, a gear library, edit suites, a rehearsal room. Start with whatever your dues can sustain. A hub that begins as a shared gear closet and a rented rehearsal night is a real hub; the fiscal engine doesn't care about square footage.
2. **A legal entity** — a co-op, association, or nonprofit that can sign a lease and hold a bank account (see Phase 2).
3. **An instance of this software** — which runs your books, your crediting, your payouts, and your growth eligibility with rules nobody, including you, can bend.

The software is the part that makes your hub different from every co-op that came before it and died of bookkeeping, favoritism, or a founder who changed. The rules that protect your members from you — and you from your members — are enforced by code you all can read.

---

## Phase 0 — Prove the demand (weeks 1–4, cost: $0)

Do not sign a lease. Do not incorporate. First:

1. **Gather your founding circle.** 5–10 people who will do the unglamorous work. Mix of crafts is better than mix of friends.
2. **Survey your city's creators.** What do they rent now, at what price, with what resentment? Your hub's initial offering is whatever appears most often in those answers.
3. **Set provisional dues.** The regional examples in this repo ($50/mo LA, ₹2,000 Mumbai, €45 Berlin) are calibrations, not rules. The test: dues × realistic member count must cover your Phase-3 operating costs with margin. If it doesn't, shrink the offering — don't inflate the member projection.
4. **Get 25+ written commitments** ("I will pay X/month for Y") before spending anything. Softer than that is a wish, not demand.

**Gate to proceed:** committed dues ≥ 1.5× your projected monthly operating cost. This ratio is what will later carry you from Critical to Healthy in the fiscal engine — hubs that skip this math are the ones the engine will (correctly) refuse to let pay out.

> **→ Do this now:** message five creators in your city this sentence: *"If a member-owned studio existed here with open books and pay decided by peer-reviewed work, would you pay $X/month for it?"* The replies are your Phase 0.

## Phase 1 — Run the software before the studio exists (week 2 onward)

Install and operate the platform on paper-trades *now*, before real money:

```bash
git clone https://github.com/Tokeloshe/studio-commons.git
cd studio-commons
cargo build --release
cargo test --all        # every economic rule, proven on your machine
STUDIO_REGION=<YOURCITY> cargo run --release
```

For the first month, mirror your founding circle's actual expenses and pledged dues into the fiscal engine as a dry run. Close a period. Read the report. You will see your hub as the engine sees it: probably **Critical** (under 3 months runway), meaning every unit of surplus would go to reserves, not payouts. That's not a failure — that's the system telling you the truth a spreadsheet would have let you hide.

Three rules you must internalize now, because they are not negotiable later:

- **Costs come first.** The engine will not distribute a cent while the period's bills are unpaid. If your hub loses money, nobody gets paid — including the platform's 1% fee, which applies only to *net profit*.
- **Health gates generosity.** Payouts to members scale with runway: nothing below 3 months of reserves, partial from 3–6, full at 6+. Explain this to every founding member before they join: *the first year mostly builds the war chest that makes every later year safe.*
- **Everything is recomputable.** Every score, payout, and audit is deterministic. Any member can re-run the books. Make this your recruiting pitch — it is the thing no traditional studio can say.

> **→ Do this now:** run the four commands above. Then enter last month's real numbers from your founding circle — pledged dues as revenue, realistic costs as expenses — close the period, and screenshot the report. Bring it to your next meeting: it's the most honest pitch deck your hub will ever have.

## Phase 2 — Incorporate and adopt the constitution (weeks 4–8)

*(What follows is a playbook, not legal advice — entity law varies by jurisdiction, and the one hour with a real co-op lawyer at the end of this phase is the actual requirement.)*

1. **Pick the entity type your jurisdiction supports:** worker/multi-stakeholder cooperative where available (best fit), else a nonprofit association or member-owned LLC. What matters is: one member interest = tied to contribution, board answerable to members, surplus distributable per your bylaws.
2. **Write the software's rules into your bylaws.** This is the crucial step that makes the code legally binding rather than advisory. Your bylaws should state:
   - Member compensation from surplus is computed by the CCI ledger (verified hours × median of ≥3 independent peer reviews), as implemented in the deployed version of this software.
   - Distributions follow the fiscal engine's health-gated waterfall; no officer may authorize a distribution the engine refuses.
   - The period-close conservation audit and CCI ledger head-hash are published to all members every period.
   - Membership is open to anyone meeting published, objective criteria (dues + code of conduct). No identity-based admission or scoring in any direction — the standards enforced by the governance module.
   - Amendments to economic rules require a member vote AND a corresponding change to the open-source deployment, so rules-as-practiced never drift from rules-as-written.
3. **Open the bank account, adopt the code of conduct, get whatever local insurance a rehearsal/production space requires.**

The AGPL license means your deployment — including any modifications — must stay open source. Members can always inspect the exact rules governing their money. That's not a burden; it's your credibility.

> **→ Do this now:** book one hour with a local co-op association or a lawyer who's formed one (many co-op federations consult founding groups for free). Bring the bylaw list above and this repo's README. That single meeting turns "software project" into "institution."

## Phase 3 — Open (month 2–4)

Start smaller than your ambition:

1. **Launch with the minimum viable offering** your committed dues sustain. Gear library + booking calendar beats an empty soundstage you can't heat.
2. **Record everything in the engine from day one.** Every due, every rental, every expense, in real time. The period-close reports become your member meetings' agenda: revenue, costs, surplus, health state, runway — one screen, no interpretation.
   **And reconcile it monthly**: a rotating member (never the bookkeeper) compares the engine's entries against the actual bank statement before the period report is accepted. The conservation audit proves the books are internally exact; the reconciliation proves they match reality. You need both — the second one is the honest answer to "what if the bookkeeper lies?", and rotating it means trust never concentrates in one person.
3. **Start the CCI ledger immediately**, including for the founding work itself. Building shelves, writing the booking system, running the open house — log it, get it peer-reviewed. When your hub reaches Healthy and the first member distribution runs, the people who built the place get paid for having built it. This is how you compensate founders *without* founder equity: the ledger remembers.
4. **Recruit reviewers across crafts.** CCI requires ≥3 independent reviews per contribution, and project collaborators can't review each other's work on that project. Small hubs should twin with another hub (or a remote circle of members) early to keep the review pool honest — the median-of-independents is your defense against both cliques and grudges.

> **→ Do this now:** pick your opening date, and log the founding work done so far as the first entries in your CCI ledger. The ledger's first block should be the story of the people who built the place.

## Phase 4 — Reach Healthy (months 4–18)

Your only economic goal for year one: **6 months of reserves at your burn rate, then 3 consecutive profitable Healthy periods.** Everything else is vanity.

- Watch the period reports. The engine will move you Critical → Rebuilding → Healthy automatically as reserves build; distributions phase in on the same schedule.
- If you hit a losing month, the engine draws reserves and pays nobody — including us. If you hit insolvency, future surplus repays the debt before anything counts as profit again. There is no way to pay members ahead of obligations, which means there is no way for your hub to quietly die the way co-ops usually do.
- When members ask why payouts are small at first, show them the runway number, not a promise. The system's honesty is your management tool.

**When you reach Healthy with a 3-period track record, two things unlock:**
1. Full member distributions — 50% of surplus by CCI merit, every period.
2. Sponsor eligibility — your expansion fund (10% of healthy surplus, plus reserve overflow) can now seed the *next* hub.

> **→ Do this now (and every month):** publish the period report, the conservation audit, and the ledger head-hash where every member sees them — a pinned post, a printout by the door, anywhere public. A hub whose members watch the runway number climb together doesn't need morale management.

## Phase 5 — Seed the next city (year 2+)

This is where your hub stops being a studio and becomes a movement:

- Your expansion fund accumulates automatically from surplus. When a founding circle in another city reaches *their* Phase-2 gate, your hub can seed them: `network.expand()` moves the capital, records the lineage, and the child hub starts life with reserves.
- The rules protect both sides: seeding never touches your reserves, requires your sustained Healthy record, and every seeded unit is conserved into exactly one child — provable by the network audit.
- The sponsor-selection ranking is deterministic (longest healthy streak first), so when multiple hubs could sponsor, the network doesn't argue about it. There is nothing to argue about.

A hub you seed carries your lineage forever in its ledger, and seeds its own children in turn. Generation by generation, the network grows exactly as fast as the model proves itself — never faster.

> **→ Do this now:** the moment your hub turns Healthy, post it in [GitHub Discussions](https://github.com/Tokeloshe/studio-commons/discussions) with your city and your numbers. Founding circles in other cities are looking for proof it works — and for sponsors. Your report is both.

---

## The operating rhythm (once running, ~2 hours/week)

| Cadence | What | Who |
|---|---|---|
| Continuous | Record revenue/expenses; members log CCI contributions | Bookkeeper role; every member |
| Weekly | Peer reviews assigned and completed | Review circle |
| Monthly | Bank-statement reconciliation against engine entries | Rotating member (never the bookkeeper) |
| Monthly | `close_period` → publish report + conservation audit + ledger head-hash | Bookkeeper, verified by any member |
| Monthly | Member meeting: the period report *is* the agenda | Everyone |
| Quarterly | Governance votes (pricing, purchases, policy) via the governance module | Everyone |
| Yearly | Re-verify standards compliance (open access, fair wage, sustainability) | Board |

## What you may change, and what you must not

**Yours to decide** (governance votes, bylaws): dues, pricing, what to build, who your officers are, local policies, which city you seed.

**Not yours to decide** — and not ours either; these invariants are what makes a hub trustworthy, and a deployment that breaks them isn't a Studio Commons hub, it's a co-op with extra steps:

- Identity-blind merit: nothing about who a member is may enter scoring, in any direction.
- Median-of-independent-peers review, with conflict-of-interest exclusion.
- Costs first, debts before profit, health-gated distributions.
- Exact conservation with published audits, every period.
- Expansion only from surplus, only with a track record, never from reserves.

## Troubleshooting the human parts

- **"Our best member is threatening to leave over their scores."** Show them the median math and the reviews. If the reviews are honest, the score is the answer. If reviews are dishonest, fix the review pool (twin with another hub) — never the score.
- **"We want to pay someone a guaranteed salary."** Fine — that's an *expense* (a fair-wage staff role), not a distribution. The engine handles it correctly: salaries are costs, paid before any profit exists. Just don't disguise distributions as salaries; your members can read the books.
- **"A rich patron wants to fund us."** Record it as revenue (a grant). It flows through the same waterfall — which means a windfall in Critical state builds your reserves rather than triggering a party. The patron should love this.
- **"We're Healthy but a member says the books are wrong."** Perfect — that's the system working. Have them recompute: the ledger, scores, and audit are deterministic. Either they find a real discrepancy (take it seriously; the hash chain will locate the tampered entry) or they've just verified the books personally. Both outcomes build the hub.

## Start today, with nothing

The zero-cost first step: fork the repo, run `cargo test --all`, and watch the full adversarial suite — around 90 tests, including simulated decades of boom-and-bust economies checked for exact conservation after every period — fail to break the economics on your own machine. Then send this document to five people in your city.

You do not need our permission. You do not need our blessing. You need a room full of people who make things and are done being taken.

---

*Questions, war stories, and hub announcements: [GitHub Discussions](https://github.com/Tokeloshe/studio-commons/discussions).*
