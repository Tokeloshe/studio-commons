# Studio Commons

[![GitHub license](https://img.shields.io/github/license/Tokeloshe/studio-commons)](https://github.com/Tokeloshe/studio-commons/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/Tokeloshe/studio-commons)](https://github.com/Tokeloshe/studio-commons/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/Tokeloshe/studio-commons)](https://github.com/Tokeloshe/studio-commons/issues)

**The creative industry has a trust problem. This is infrastructure that removes the need for trust.**

Studio Commons is community-owned studio infrastructure where crediting, payment, solvency, and growth are enforced by open-source code instead of promises — every rule deterministic, every decision recomputable by any member, and every known way to cheat either blocked or made detectable. Rust core; AGPL-3.0, so every deployment (including hosted ones) must publish its source.

> ### Want a studio like this in your city?
> **You don't need our permission — that's the point.** The complete playbook, from zero commitments to a running hub that seeds the next city, is here:
> ## **[Founding a Hub: The Complete Guide](FOUNDING_A_HUB.md)**

---

## The Problems This Kills

Every working filmmaker, musician, and creator knows these. They are not bugs in the industry — they *are* the industry. Each one is eliminated here by a specific, tested mechanism, not a policy document.

### 1. "Hollywood accounting" — profitable films that never pay
*Forrest Gump*, on paper, lost money. Studios route revenue through opaque subsidiaries until net-profit participants get nothing. It works because the books are private and the definitions are theirs.

**Killed by:** the fiscal engine closes every period with an **exact integer conservation identity** — revenue = expenses + fees + distributions + funds held, to the unit. The audit function is public code. If a single cent is unaccounted for, the audit fails and every member can see it. "The books" are not a claim; they're a recomputable proof.

### 2. Lost credits and stolen residuals
Work-for-hire contracts, forgotten credits, residual checks that stop coming, decades of union fights over streaming revenue. The person who did the work has no durable, enforceable record that they did it.

**Killed by:** the Creative Contribution Index (CCI) — a **tamper-evident, hash-chained ledger** of who did what. Your contribution record is permanent: if a project earns money in year ten, your share is recomputed from the same ledger, deterministically, and pays out at the same proportion. Rewriting history invalidates the chain and the built-in audit catches it. No negotiating, no "we lost the paperwork."

### 3. Credit and pay decided by politics, not work
Who you know, who likes you, who takes credit for your work — the industry runs on favoritism in every direction.

**Killed by:** merit math that is **identity-blind by construction**. Your score is verified hours × the *median* of at least 3 independent peer reviews. Nothing about who you are enters the formula — no bonuses or penalties for anyone. The median means one shill review or one hostile saboteur cannot move your score. You can't review yourself, you can't review your own project's collaborators, you can't vote twice — all enforced in code, all battle-tested against exactly those attacks.

### 4. Studios that collapse — or extract everything
Traditional model: an owner invests $5M, extracts $10M, the crew gets day rates. Or the opposite failure: a well-meaning co-op distributes everything and dies the first bad quarter, because generosity ran ahead of solvency.

**Killed by:** a costs-first fiscal engine that makes both failure modes structurally impossible:
- **Nothing is "profit" until every bill is paid.** Loss periods pay no fees and no distributions — there is no profit to distribute.
- **Debts before profit.** Unpaid obligations from bad periods are settled out of future surplus before the waterfall runs.
- **Survival before generosity.** Allocation follows the hub's runway (months of reserves at current burn): under 3 months, every unit of surplus rebuilds reserves; 3–6 months, half does; at 6+ months, full distribution resumes — 50% to members by merit, 20% local reinvestment, 10% to expansion, ~20% to reserves.
- **Prudence has a cap.** Reserves beyond 12 months of runway overflow into expansion — money beyond safety builds the next studio instead of hoarding.

A hub running this code *cannot* pay members while its rent goes unpaid, and *cannot* sit on a fortune while claiming poverty. Both are compile-time-style guarantees about money.

### 5. Gatekept access
Professional infrastructure — stages, gear, post suites — is priced for incumbents and gatekept by them.

**Killed by:** cost-share membership with open-access standards enforced on every licensed hub: membership decided purely on published, objective criteria; fair wages meeting local standards; sustainability targets. No quotas on who you are, guarantees on how you're treated — the same deal in every city on Earth.

### 6. Growth that outruns the model — or never happens
Franchises expand on hype and implode; co-ops stay single-city forever because nobody can safely fund the second location.

**Killed by:** the network layer, where **growth flows from proven strength**:
- A new hub is seeded only from a sponsor's expansion fund — reserves are never touched.
- The sponsor must be Healthy *now* and have 3 consecutive profitable, Healthy periods. One good month is not a track record; one loss resets the clock.
- The sponsor is chosen by a deterministic ranking (longest healthy streak, then largest fund) that any member can recompute — no committee to lobby.
- Hubs are financially firewalled: a failing hub cannot draw one unit from any other hub's reserves.
- Every seed is conserved: the network audit proves money leaving sponsors equals founding capital arriving in children, exactly.

The result is a network that self-replicates from genuine surplus — every new hub exists because the model demonstrably worked somewhere else first.

> **Recognize these problems?** They end when someone in your city decides to end them. **[Start with Phase 0 of the founding guide →](FOUNDING_A_HUB.md#phase-0--prove-the-demand-weeks-14-cost-0)** — it costs nothing and takes four weeks to find out if your city is ready.

---

## Why You Can Actually Trust This

Claims like the above are cheap. Here's what backs them:

- **It's all code, and it's all open.** AGPL-3.0 means every deployment — including hosted ones — must publish its source. There is no proprietary fork where the rules quietly change.
- **Deterministic everything.** Scores, payouts, sponsor rankings, audits: same inputs, same outputs, byte for byte. Any member, anywhere, can recompute any decision and check it.
- **Battle-tested, adversarially.** Alongside the unit tests, a dedicated attack suite tries to break the system the way a bad actor would: shill reviews, saboteur reviews, double votes, review rings, entry splitting, hour-cap probing, NaN injection, rounding attacks up to `u128::MAX`, ledger tampering, bleed-the-hub-dry sequences, flash-prosperity expansion attempts, and deterministic multi-hundred-period boom/bust fuzz economies that assert conservation after every single simulated period. Around 90 tests in total — and several of those attacks found real flaws during development. The fixes *and* the attacks that found them are both in the repo, because a system that hides its failure history is asking you to trust it, and the whole point here is that you shouldn't have to.
- **Exact integer arithmetic.** All money is integer minor units with basis-point math. No floating-point drift, no rounding dust silently lost — remainders are routed to reserves by design.

Run the proof yourself:

```bash
cargo test --all
```

> **Two minutes, zero cost, no signup:** clone the repo, run the command above, and watch the full attack suite fail to break the economics on your own machine. Then read **[the founding guide](FOUNDING_A_HUB.md)** and decide what your city deserves.

### What this does NOT solve — read before you flame us

A system that claims to fix trust owes you its limits up front:

- **The oracle problem.** The conservation audit proves the books are *internally* consistent — it cannot prove that the revenue your bookkeeper typed in matches reality. Code can't see your bank account. Mitigation: the founding guide mandates monthly reconciliation of engine inputs against bank statements by a rotating member, and the deterministic books make any divergence easy to localize once spotted. But the entry point for real-world data is, and always will be, a human.
- **Majority collusion beats the median.** The median-of-3+ review defeats one shill or one saboteur, and conflict-of-interest rules make rings expensive — but if a majority of a contribution's reviewers collude, they win. Small hubs are most exposed, which is why the guide tells them to twin their review pools with another hub. Honest framing: we made capture *costly and detectable* (the Gini capture-warning exists for exactly this), not impossible. Nothing makes it impossible.
- **Hours are attested, not surveilled.** The engine caps and audits hour claims, and peer review scores the *output* — but nobody's timing your keystrokes. A community whose members grossly inflate hours has a culture problem no ledger fixes.
- **The ledger hash is not yet cryptographic.** The chain currently uses a fast non-cryptographic hash — fine for catching accidental corruption and casual tampering, forgeable by a determined attacker with storage access. The design anticipates swapping in SHA-256 and anchoring head-hashes externally (that's why `head_hash()` exists); until that lands, the hash chain is a smoke detector, not a vault.
- **Code isn't law until your bylaws say so.** The engine refusing an improper distribution only binds your co-op if your bylaws bind you to the engine. The founding guide's Phase 2 exists precisely to close this gap — skip it and you have great software and no institution.

If you find a hole not on this list, please [open an issue](https://github.com/Tokeloshe/studio-commons/issues). Breaking this system in public is a contribution.

---

## How It Fits Together

| Layer | Question it answers | Guarantee |
|---|---|---|
| **CCI** (`src/cci`) | Who earned what? | Identity-blind merit on a tamper-evident ledger |
| **Fiscal engine** (`src/economics`) | What's safe to pay? | Costs first, debts before profit, exact conservation |
| **Network** (`src/network`) | Where does growth go? | Strongest proven hub sponsors, firewalled, seed-conserved |
| **Governance** (`src/governance`) | Who decides policy? | DAO voting, licensed hubs, open-access standards |
| **Payments** (`src/payments`) | How does money move? | Multi-currency processing, XRPL founder fee |
| **Treasury** (`src/treasury`) | What do idle funds do? | DeFi yield modeling (simulated), risk analytics, carbon tracking |
| **Membership** (`src/membership`) | Who's in? | Portable global IDs, regional pricing |
| **Compliance** (`src/compliance`) | Is it legal here? | Per-jurisdiction adapters (GDPR, IT Act, IRS, …) |
| **Analytics** (`src/analytics`) | Is it staying fair? | Identity-blind reward-concentration (Gini) capture detection |

### A month in the life of a hub

1. Revenue comes in: stage rentals, memberships, project services — recorded to the period.
2. Members log contributions to the CCI ledger; independent peers review them; medians settle scores.
3. The period closes: expenses paid first, past debts settled, then the health-gated waterfall — founder's fee (1% of *net profit*), member distributions by CCI share, reinvestment, expansion, reserves.
4. The conservation audit publishes: every unit accounted for, recomputable by anyone.
5. When the hub has earned it — 6+ months runway, 3 straight healthy profitable periods — its expansion fund can seed the next city.

### A film, end to end

Maria directs an indie feature at her hub. Every crew member's hours land on the CCI ledger, peer-reviewed as they go. The film wraps, gets distribution, earns $500K. The waterfall runs: the hub's costs on the project are covered, then the crew's shares are computed from the same ledger entries — Maria's 270 points against the project's total — and paid. Five years later a streamer licenses it: **the same ledger recomputes the same proportions and everyone is paid again.** Nobody negotiates, nobody is forgotten, nobody's cousin in accounting decides the film "lost money."

> **Want Maria's ending for your own crew?** The path from "room full of frustrated creators" to that first automated payout is mapped step by step: **[Founding a Hub →](FOUNDING_A_HUB.md)**

---

## Founder's Fee — full disclosure

The reference deployment carries a 1% fee on **net profits** (never on revenue; a loss-making period pays nothing) to fund ongoing development:

- **XRP Wallet**: `rf82s1CDagppvM6ATqc1nSrL6GackzHJrm` · **Memo**: `2621443948`
- **Verification**: `PaymentsSystem::verify_founder_config()` — a compile-time constant; every fee transaction is logged in the same auditable books as everything else.

Being straight with you, because an anti-extraction project owes you this paragraph: yes, this is a fee to the project's creator, and no, it is not technically immutable — this is AGPL code, and a fork can strip it in one line. We're not going to pretend otherwise and hope you don't check. It stays visible in the source, subject to the same exact math as every other flow, and capped at 1% of *profit* — if your hub never profits, we never see a cent. If you run a fork without it, the license permits that; if you keep it, you're funding the commons this came from. We think that trade, stated plainly, survives scrutiny better than any platform's fee page you've ever read.

## Tech Stack — what's real today vs. planned

**Working and tested now (this is the part you can verify with `cargo test --all`):**
- **Rust workspace, 11 crates**: the CCI merit ledger, the fiscal engine, the network layer, governance voting, payments accounting, membership, analytics — the complete economic rule system, with the adversarial test suite.

**Present as interfaces/simulations, pending real integration (open the code; the stubs are commented as stubs):**
- **XRPL payments**: fee transactions are computed and logged with correct amounts, but not yet submitted to the ledger — XRPL SDK integration is the bridge to build.
- **DeFi treasury** (Aave/Compound/Yearn) and **compliance adapters**: modeled with realistic structure, not yet wired to live protocols or legal databases.
- **Substrate governance anchoring, IPFS storage, React dashboard**: designed for, not started.

We list these separately because nothing erodes trust faster than discovering a stub behind a claim. The economic core came first, deliberately: it's the part the industry gets wrong on purpose, and the part that had to be provably right before anything touches real money.

## Installation

Prerequisites: Rust 1.70+ ([install](https://rustup.rs/)). Windows users: see [WINDOWS_INSTALL.md](WINDOWS_INSTALL.md).

```bash
git clone https://github.com/Tokeloshe/studio-commons.git
cd studio-commons
cargo build --release
cargo test --all      # run the full battle-test suite
cargo run --release
```

Configure your hub's region:

```bash
export STUDIO_REGION=LA   # LA, NYC, MUMBAI, BERLIN, LAGOS, ...
cargo run --release
```

## Project Structure

```
studio-commons/
├── Cargo.toml              # Workspace configuration
├── src/
│   ├── main.rs            # Application entry point
│   ├── cci/               # Merit ledger: peer-reviewed, hash-chained, identity-blind
│   ├── economics/         # Fiscal engine: costs-first waterfall, runway health, expansion fund
│   ├── network/           # Hub fleet: sponsor ranking, firewalled seeding, lineage
│   ├── governance/        # DAO voting, licensing, open-access standards
│   ├── payments/          # Revenue processing + founder's fee
│   ├── treasury/          # DeFi integration, risk, carbon tracking
│   ├── membership/        # Global member management
│   ├── analytics/         # Fairness (Gini/capture) + impact forecasting
│   ├── compliance/        # Per-jurisdiction legal adapters
│   └── utils/             # Shared types
```

## Status & Roadmap

**Now**: the full economic core — merit ledger, fiscal engine, network layer, governance, payments — implemented and battle-tested in Rust. This is the part that had to be right first, because it's the part the industry gets wrong on purpose.

**Next**:
- Web dashboard (React) so members see their ledger, shares, and hub health live
- XRPL + Substrate integration to anchor ledger head-hashes and execute real payouts
- LA pilot hub deployment
- Mobile apps; additional jurisdiction adapters

**The goal**: a global network of community-owned studios where the second hub is seeded by the first hub's proven surplus, the tenth by the strongest of the nine — and every artist in every one of them holds a permanent, tamper-evident claim on the value of their work.

> **The first hub in the network could be yours.** Every generation of this network will trace its lineage to whoever moves first. **[The founding guide is here →](FOUNDING_A_HUB.md)**

## Contributing

Contributions welcome — the standards are the same as the code's: tests for every feature, `cargo test --all` green, and any change to economic logic must come with battle tests that attack it.

1. Fork → feature branch → tests → PR.
2. High-impact areas: jurisdiction adapters, the React dashboard, XRPL/Substrate integration, localization.

## License

**AGPL-3.0** — use it, modify it, deploy it; but every deployment, including network services, must publish its source under the same license. Community infrastructure that cannot be quietly enclosed.

## Contact

- **Creator**: [@e_honiball](https://x.com/e_honiball) on X
- **Issues**: [GitHub Issues](https://github.com/Tokeloshe/studio-commons/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Tokeloshe/studio-commons/discussions)

---

**Built for a new age of global creation.**

*"The industry runs on trust and breaks it for profit. Infrastructure shouldn't need trust."*

**[Found a hub](FOUNDING_A_HUB.md)** · **[Run the proof](#why-you-can-actually-trust-this)** · **[Join the discussion](https://github.com/Tokeloshe/studio-commons/discussions)**
