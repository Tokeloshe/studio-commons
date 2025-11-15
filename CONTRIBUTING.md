# Contributing to Studio Commons

Thank you for your interest in contributing to Studio Commons! This document provides guidelines and instructions for contributing to this global creative infrastructure platform.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Areas for Contribution](#areas-for-contribution)

## Code of Conduct

Studio Commons is committed to providing a welcoming and inclusive environment for all contributors, regardless of:
- Gender identity and expression
- Sexual orientation
- Disability
- Physical appearance
- Race or ethnicity
- Age
- Religion or belief
- Geographic location

We expect all contributors to:
- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Assume good intentions

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/studio-commons.git
   cd studio-commons
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/Tokeloshe/studio-commons.git
   ```
4. **Keep your fork synced**:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

## Development Setup

### Prerequisites
- Rust 1.70+ ([install](https://rustup.rs/))
- Cargo (comes with Rust)
- Git

### Build and Test
```bash
# Build the project
cargo build

# Run tests
cargo test --all

# Run a specific module's tests
cargo test -p payments

# Run with logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint code
cargo clippy
```

## How to Contribute

### Reporting Bugs
1. Check if the bug has already been reported in [Issues](https://github.com/Tokeloshe/studio-commons/issues)
2. If not, create a new issue with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs actual behavior
   - System information (OS, Rust version)
   - Relevant logs or error messages

### Suggesting Enhancements
1. Check existing [Issues](https://github.com/Tokeloshe/studio-commons/issues) and [Discussions](https://github.com/Tokeloshe/studio-commons/discussions)
2. Create a new issue or discussion with:
   - Clear description of the enhancement
   - Use cases and benefits
   - Potential implementation approach
   - Impact on existing functionality

### Contributing Code

1. **Create a branch** for your work:
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

2. **Make your changes** following our coding standards

3. **Write tests** for new functionality

4. **Run tests** to ensure nothing breaks:
   ```bash
   cargo test --all
   ```

5. **Commit your changes** with clear messages:
   ```bash
   git commit -m "Add feature: description of what you did"
   ```

6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request** on GitHub

## Coding Standards

### Rust Best Practices
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### Code Style
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and reasonably sized
- Use Result types for error handling
- Avoid unwrap() in production code (use ? or proper error handling)

### Module Guidelines
```rust
// Good: Clear documentation
/// Calculate the founder's fee from net profits
///
/// This is a perpetual 1% fee hardcoded to support ongoing development
pub fn calculate_founder_fee(profit: u128) -> u128 {
    (profit as f64 * FOUNDER_FEE_PERCENTAGE / 100.0) as u128
}

// Bad: No documentation, unclear purpose
pub fn calc(x: u128) -> u128 {
    x / 100
}
```

### Important: Founder's Fee Integrity
**CRITICAL**: The founder's fee configuration must remain immutable:
- XRP Wallet: `rf82s1CDagppvM6ATqc1nSrL6GackzHJrm`
- Memo: `2621443948`
- Percentage: 1%

Do not modify these values in `src/payments/src/lib.rs`. Pull requests that alter this configuration will be rejected.

## Testing

All contributions must include appropriate tests.

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = your_function(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Test Coverage
- Write unit tests for all new functions
- Write integration tests for module interactions
- Test edge cases and error conditions
- Aim for >80% code coverage

### Running Tests
```bash
# All tests
cargo test --all

# Specific module
cargo test -p governance

# With output
cargo test -- --nocapture

# Single test
cargo test test_founder_fee_calculation
```

## Documentation

### Code Documentation
- Document all public APIs with `///` doc comments
- Include examples in documentation
- Explain parameters, return values, and errors

```rust
/// Process global revenue and allocate according to the 50/30/20 split
///
/// Automatically deducts 1% founder's fee before distribution.
///
/// # Arguments
/// * `stream` - Type of revenue stream (rental, streaming, etc.)
/// * `amount` - Total revenue amount in smallest currency unit
/// * `currency` - Currency type (USD, EUR, XRP, etc.)
///
/// # Returns
/// Allocation breakdown showing member distribution, reinvestment, reserves, and founder's fee
///
/// # Example
/// ```
/// let allocation = payments.process_global_revenue(
///     StreamType::Rental,
///     100000,
///     Currency::USD
/// )?;
/// ```
pub fn process_global_revenue(
    &mut self,
    stream: StreamType,
    amount: u128,
    currency: Currency,
) -> Result<Allocation>
```

### README Updates
Update README.md if your changes:
- Add new features
- Change installation steps
- Modify usage instructions
- Add new dependencies

## Pull Request Process

1. **Ensure your PR**:
   - Has a clear, descriptive title
   - References related issues
   - Includes tests
   - Updates documentation
   - Passes all CI checks

2. **PR Description should include**:
   - What the change does
   - Why it's needed
   - How it works
   - Any breaking changes
   - Screenshots (if UI changes)

3. **Review Process**:
   - Maintainers will review your PR
   - Address feedback constructively
   - Make requested changes
   - Keep discussions focused and professional

4. **Merging**:
   - PRs require approval from maintainers
   - All tests must pass
   - Code must meet quality standards
   - Documentation must be updated

## Areas for Contribution

### High Priority
- **Frontend Development**: React/Vue dashboard for web interface
- **Smart Contracts**: Substrate pallets and XRPL integration
- **Regional Adapters**: Compliance modules for new jurisdictions
- **AI Integration**: Enhanced ML models for CCI and analytics
- **Documentation**: Tutorials, guides, and API documentation

### Module-Specific Contributions
- **Governance**: DAO improvements, voting mechanisms
- **Treasury**: DeFi protocol integrations, risk models
- **CCI**: Bias detection algorithms, contribution types
- **Production**: AI ethics frameworks, virtual production tools
- **Membership**: Cross-hub portability, tier benefits
- **Analytics**: Predictive models, impact metrics
- **Compliance**: New jurisdiction support, union integration

### Infrastructure
- CI/CD improvements
- Docker containerization
- Kubernetes deployment configs
- Monitoring and observability
- Performance optimization

### Translation & Localization
- UI/UX in multiple languages
- Regional documentation
- Cultural adaptation

## Regional Contributions

Studio Commons is a global platform. We especially welcome contributions that:
- Add support for new regions
- Improve compliance with local laws
- Adapt to cultural contexts
- Support local currencies
- Integrate with regional services

### Adding a New Region
1. Add region to `utils::Region` enum
2. Update `compliance` module with local laws
3. Add currency support if needed
4. Update `membership` with localized dues
5. Test thoroughly
6. Document regional specifics

## Questions or Need Help?

- **Discussions**: [GitHub Discussions](https://github.com/Tokeloshe/studio-commons/discussions)
- **Issues**: [GitHub Issues](https://github.com/Tokeloshe/studio-commons/issues)
- **Contact**: [@e_honiball](https://x.com/e_honiball) on X

## License

By contributing to Studio Commons, you agree that your contributions will be licensed under the AGPL-3.0 license.

---

**Thank you for helping build the future of community-owned creative infrastructure!**

*Together, we're reclaiming creative spaces for artists worldwide.*
