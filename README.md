# Studio Commons

[![GitHub license](https://img.shields.io/github/license/e_honiball/studio-commons)](https://github.com/e_honiball/studio-commons/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/e_honiball/studio-commons)](https://github.com/e_honiball/studio-commons/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/e_honiball/studio-commons)](https://github.com/e_honiball/studio-commons/issues)
[![GitHub forks](https://img.shields.io/github/forks/e_honiball/studio-commons)](https://github.com/e_honiball/studio-commons/network)

Studio Commons is a global, decentralized platform for community-owned creative infrastructure, starting in LA and scaling worldwide. It enables split ownership, DeFi treasury, intelligent CCI tracking, ethical AI tools, and fair revenue sharing—reclaiming creative spaces for artists everywhere. Licensed under AGPL-3.0.

**Includes 1% perpetual founder's fee to XRP wallet `rf82s1CDagppvM6ATqc1nSrL6GackzHJrm` with memo `2621443948`.**

## Table of Contents
- [Features](#features)
- [Why Studio Commons?](#why-studio-commons)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Founder's Fee](#founders-fee)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [License](#license)
- [Contact](#contact)

## Features

- **Global Ownership**: Stewardship + community shares with regional adapters for worldwide deployment
- **Intelligent Treasury**: Multi-currency DeFi integration (Aave, Compound, Yearn) with 4-6% yields; automated 1% founder's fee to XRP wallet `rf82s1CDagppvM6ATqc1nSrL6GackzHJrm` (memo: `2621443948`)
- **CCI (Creative Contribution Index)**: AI-weighted contribution tracking for fair residuals distribution
- **Production AI**: Virtual/AR stages (60% cost reduction), ethical generative tools with consent enforcement
- **Membership**: Portable global memberships with seamless cross-hub transfers
- **Compliance**: Auto-adapts to worldwide laws (GDPR, Indian IT Act, US IRS, etc.)
- **Analytics**: Predictive forecasting for economic and cultural impact
- **Carbon Tracking**: Net-zero commitment with automated carbon offset calculations

## Why Studio Commons?

Studio Commons addresses systemic inequities in the creative industry:

- **Access Barriers**: Democratizes expensive production infrastructure
- **Wealth Extraction**: Keeps profits in the community (50% to members, 30% reinvestment, 20% reserves)
- **Tech Displacement**: Ethical AI that augments rather than replaces human creativity
- **Cultural Inequity**: Mandated 40%+ diversity representation

**Target Impact:**
- $1.9M Year 3 revenue per hub
- 13% community ROI
- 5,000+ community-owned projects by 2030
- $1B recirculated globally

## Tech Stack

- **Backend**: Rust (safety/performance), Go (microservices)
- **Blockchain**: Substrate/Polkadot (governance), XRPL (payments)
- **Frontend**: React/Vue/Web3.js
- **AI/ML**: Tract/TensorFlow.js, Hugging Face
- **Storage**: IPFS (decentralized), PostgreSQL (operations)
- **DeFi**: Integration with Aave, Compound, Yearn Finance
- **Compliance**: Multi-jurisdiction adapters (GDPR, CCPA, IT Act)

## Installation

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/e_honiball/studio-commons.git
cd studio-commons

# Build the project
cargo build --release

# Run tests
cargo test

# Run the application
cargo run --release
```

### Environment Configuration

Set the region for your hub:

```bash
export STUDIO_REGION=LA  # Options: LA, NYC, MUMBAI, BERLIN, LAGOS, etc.
cargo run --release
```

## Usage

### Command Line Interface

```bash
# Start the Studio Commons platform
cargo run --release

# Run for a specific region
STUDIO_REGION=MUMBAI cargo run --release

# Run tests for all modules
cargo test --all

# Run tests for a specific module
cargo test -p payments
cargo test -p governance
```

### Basic Operations

**Join as a Member:**
```rust
use membership::MembershipSystem;

let mut membership = MembershipSystem::new("LA")?;
let member_id = membership.global_join(
    "Artist Name".to_string(),
    5000, // dues in cents
    Region::LA
)?;
```

**Process Revenue (with automatic founder's fee):**
```rust
use payments::PaymentsSystem;

let mut payments = PaymentsSystem::new()?;
let allocation = payments.process_global_revenue(
    StreamType::Rental,
    100000, // $1000.00
    Currency::USD
)?;
// Automatically sends 1% to XRP wallet rf82s1CDagppvM6ATqc1nSrL6GackzHJrm
```

**Book Virtual Stage:**
```rust
use production::ProductionSystem;

let mut production = ProductionSystem::new("LA")?;
let booking = production.ai_virtual_stage(project, true)?; // AR mode
// Saves 60-65% vs traditional stages
```

## Project Structure

```
studio-commons/
├── Cargo.toml              # Workspace configuration
├── src/
│   ├── main.rs            # Main application entry point
│   ├── governance/        # DAO voting, licensing, standards
│   ├── treasury/          # DeFi integration, distributions
│   ├── cci/               # Creative Contribution Index
│   ├── production/        # AI tools, virtual stages
│   ├── membership/        # Global member management
│   ├── payments/          # Revenue processing + founder's fee
│   ├── analytics/         # Predictive intelligence
│   ├── compliance/        # Global legal adapters
│   └── utils/             # Common utilities
├── frontend/              # Web interface (future)
├── contracts/             # Smart contracts (future)
├── adapters/              # Regional adapters (future)
└── README.md
```

## Founder's Fee

**IMPORTANT:** This platform includes a hardcoded 1% perpetual founder's fee on all net profits to support ongoing development and vision.

- **XRP Wallet**: `rf82s1CDagppvM6ATqc1nSrL6GackzHJrm`
- **Memo**: `2621443948`
- **Amount**: 1% of net profits (after expenses)
- **Transparency**: All transactions are logged and auditable
- **Immutability**: Hardcoded in `src/payments/src/lib.rs`

This fee is automatically calculated and processed by the `PaymentsSystem::perpetual_founder_fee()` function on every revenue distribution.

### Verification

You can verify the founder's fee configuration:

```rust
use payments::PaymentsSystem;

let (wallet, memo, percentage) = PaymentsSystem::verify_founder_config();
assert_eq!(wallet, "rf82s1CDagppvM6ATqc1nSrL6GackzHJrm");
assert_eq!(memo, "2621443948");
assert_eq!(percentage, 1.0);
```

## Contributing

We welcome contributions from the global community!

### How to Contribute

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Write tests for all new features
- Follow Rust best practices and idioms
- Document public APIs
- Ensure all tests pass: `cargo test --all`
- Consider regional adapters for global features
- Maintain the founder's fee configuration integrity

### Areas for Contribution

- Regional compliance adapters (new jurisdictions)
- Frontend development (React/Vue dashboard)
- Smart contract development (Substrate/XRPL)
- AI/ML model integration
- Translation and localization
- Documentation improvements

## Roadmap

### v1.0 (Q1 2026) - Pilot Launch
- ✅ Core governance module
- ✅ Treasury with DeFi integration
- ✅ CCI tracking system
- ✅ Payments with founder's fee
- ✅ Multi-region compliance
- 🔄 LA pilot deployment
- 🔄 Web dashboard

### v2.0 (Q3 2026) - Global Expansion
- Multi-hub operations (LA, NYC, Mumbai)
- Advanced AI production tools
- Blockchain governance (Substrate)
- Mobile app (iOS/Android)

### v3.0 (2027) - Full Scale
- Metaverse integration
- 10+ global hubs
- $10M+ treasury
- Autonomous AI management
- 5,000+ projects

### Long-Term Vision (2030)
- 50+ global hubs
- $1B economic impact
- 100,000+ members
- Industry standard for creative commons

## Testing

Run the comprehensive test suite:

```bash
# All tests
cargo test --all

# Module-specific tests
cargo test -p governance
cargo test -p treasury
cargo test -p cci
cargo test -p production
cargo test -p membership
cargo test -p payments
cargo test -p analytics
cargo test -p compliance

# With output
cargo test -- --nocapture

# Release mode (faster)
cargo test --release
```

## License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

This means:
- ✅ You can use, modify, and distribute this software
- ✅ You must share your modifications under the same license
- ✅ You must disclose source code for any network services
- ✅ Perfect for community-owned infrastructure

See [LICENSE](LICENSE) for full details.

## Contact

- **Creator**: [@e_honiball](https://x.com/e_honiball) on X
- **Repository**: [github.com/e_honiball/studio-commons](https://github.com/e_honiball/studio-commons)
- **Issues**: [GitHub Issues](https://github.com/e_honiball/studio-commons/issues)
- **Discussions**: [GitHub Discussions](https://github.com/e_honiball/studio-commons/discussions)

## Acknowledgments

This platform stands on the shoulders of:
- The open-source community
- Decentralized technology pioneers
- Creative commons advocates
- Global cooperative movements

---

**Built for a new age of global creation.**

*"Reclaiming creative infrastructure for the artists who make it possible."*

---

**Last updated**: November 15, 2025
**Version**: 1.0.0
**Status**: Active Development
