# Xorion Developer Guide

Complete guide for building with and contributing to Xorion.

## Table of Contents

1. [Building from Source](#building-from-source)
2. [Running Tests](#running-tests)
3. [Creating dApps](#creating-dapps)
4. [Contributing Guidelines](#contributing-guidelines)
5. [Code Style](#code-style)
6. [PR Process](#pr-process)

---

## Building from Source

### Prerequisites

**Required:**
- Rust 1.75+ (stable)
- Git
- Node.js 18+ (for some tooling)
- pkg-config (Linux/macOS)
- libssl-dev (Linux)

**Optional:**
- Docker & Docker Compose
- VS Code with rust-analyzer
- cargo-edit, cargo-audit

### Installation

#### Clone Repository

```bash
git clone https://github.com/IDevNation/xorion-web3-os.git
cd xorion-web3-os
```

#### Install Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup default stable
```

#### Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev cmake protobuf-compiler
```

**macOS:**
```bash
xcode-select --install
brew install pkg-config openssl cmake protobuf
```

**Windows:**
```powershell
# Install Visual Studio Build Tools with C++ workload
# Install OpenSSL from https://slproweb.com/products/Win32OpenSSL.html
```

#### Build Project

```bash
# Debug build (faster, for development)
cargo build

# Release build (optimized)
cargo build --release

# Build specific crate
cargo build -p xorion-sdk

# Build all examples
cargo build --examples
```

#### Run Examples

```bash
# Run demo example
cargo run --example demo

# Run wallet CLI
cargo run -p xorion-cli

# Run GUI (requires display)
cargo run -p xorion-gui
```

---

## Running Tests

### Run All Tests

```bash
cargo test --workspace
```

### Run Tests for Specific Crate

```bash
cargo test -p xorion-sdk
cargo test -p xorion-zk
cargo test -p xorion-ipfs
```

### Run Specific Test

```bash
cargo test test_wallet_creation
cargo test --test integration_tests
```

### Run Tests with Output

```bash
cargo test -- --nocapture
cargo test -- --show-output
```

### Run Tests with Coverage

Install tarpaulin:
```bash
cargo install cargo-tarpaulin
```

Run coverage:
```bash
cargo tarpaulin --workspace --out Html
```

View `tarpaulin-report.html` in browser.

### Integration Tests

```bash
# Run integration tests only
cargo test --test '*'

# Run with local blockchain
cargo test --features integration
```

### Performance Tests

```bash
# Run benchmarks (requires nightly)
rustup default nightly
cargo bench

# Back to stable
rustup default stable
```

---

## Creating dApps

### Development Environment Setup

1. **Install Xorion SDK**

```bash
cargo add xorion-sdk
cargo add xorion-core  # For WASM dApps
```

2. **Set Up Project**

```bash
# Create new Rust project
cargo new my-dapp
cd my-dapp

# Add dependencies
cargo add xorion-sdk ethers tokio serde serde_json
```

3. **Configure for WASM (if needed)**

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]

[target.wasm32-unknown-unknown.dependencies]
wasm-bindgen = "0.2"
getrandom = { version = "0.2", features = ["js"] }
```

### Hello World dApp

```rust
use xorion_sdk::{Wallet, EthereumProvider};
use ethers::types::Address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize wallet
    let wallet = Wallet::from_mnemonic("your mnemonic phrase here")?;
    
    // Get Ethereum address
    let eth_address = wallet.eth_address().await?;
    println!("My address: {}", eth_address);
    
    // Connect to provider
    let provider = EthereumProvider::new("https://mainnet.infura.io/v3/YOUR_KEY");
    
    // Get balance
    let balance = provider.get_balance(eth_address).await?;
    println!("Balance: {} ETH", ethers::utils::format_ether(balance));
    
    Ok(())
}
```

### WASM dApp Template

```rust
use wasm_bindgen::prelude::*;
use xorion_core::{dapp_main, WalletBridge};

#[dapp_main]
struct MyApp {
    bridge: WalletBridge,
}

impl MyApp {
    pub async fn get_balance(&self) -> JsValue {
        let address = self.bridge.get_eth_address().await;
        let balance = self.bridge.get_balance(address).await;
        JsValue::from_str(&balance.to_string())
    }
    
    pub async fn send_transaction(&self, to: String, amount: u64) -> JsValue {
        match self.bridge.send_eth(to, amount).await {
            Ok(tx_hash) => JsValue::from_str(&tx_hash),
            Err(e) => JsValue::from_str(&format!("Error: {}", e)),
        }
    }
}
```

### dApp Permissions

Define required permissions in `dapp.toml`:

```toml
[dapp]
name = "My DApp"
version = "1.0.0"

[permissions]
wallet_access = true
transaction_signing = true
ipfs_storage = true
network_access = ["ethereum", "solana"]
filesystem_read = false
filesystem_write = false
```

### Deploy to IPFS

```bash
# Build for production
cargo build --release --target wasm32-unknown-unknown

# Upload to IPFS
xorion ipfs add target/wasm32-unknown-unknown/release/my_dapp.wasm

# Pin for persistence
xorion ipfs pin <CID>
```

### Testing dApps

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use xorion_sdk::test_utils::*;
    
    #[tokio::test]
    async fn test_get_balance() {
        let app = MyApp::new_test();
        let balance = app.get_balance().await;
        assert!(balance.as_string().is_some());
    }
}
```

---

## Contributing Guidelines

### Ways to Contribute

- 🐛 Bug reports and fixes
- ✨ New features
- 📚 Documentation improvements
- 🎨 UI/UX enhancements
- 🧪 Test coverage
- 🔒 Security audits
- 🌍 Translations

### Getting Started

1. **Fork the repository**
2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/xorion-web3-os.git
   cd xorion-web3-os
   ```
3. **Create branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make changes**
5. **Test thoroughly**
6. **Submit PR**

### Issue Labels

- `good first issue`: Perfect for newcomers
- `help wanted`: Need community assistance
- `bug`: Something isn't working
- `enhancement`: New feature request
- `documentation`: Docs improvements
- `security`: Security-related

---

## Code Style

### Rust Code Style

We follow standard Rust conventions with rustfmt:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

**Key Guidelines:**

- Use `snake_case` for functions and variables
- Use `PascalCase` for types and structs
- Use `SCREAMING_SNAKE_CASE` for constants
- 4-space indentation
- Max line length: 100 characters
- Document public APIs with rustdoc

Example:
```rust
/// Creates a new wallet from mnemonic phrase.
/// 
/// # Arguments
/// * `mnemonic` - BIP-39 recovery phrase (12-24 words)
/// * `password` - Optional BIP-39 passphrase
/// 
/// # Returns
/// * `Result<Wallet, WalletError>` - Created wallet or error
/// 
/// # Example
/// ```
/// let wallet = Wallet::from_mnemonic("abandon abandon ...")?;
/// ```
pub fn from_mnemonic(mnemonic: &str, password: Option<&str>) -> Result<Wallet, WalletError> {
    // Implementation
}
```

### Clippy Linting

```bash
# Run linter
cargo clippy --workspace -- -D warnings

# Auto-fix where possible
cargo clippy --fix --allow-dirty
```

**Common Rules:**
- Avoid `unwrap()` - use `expect()` with message
- Use `if let` instead of `match` for single patterns
- Prefer iterators over indexing
- Use `?` operator for error propagation

### Commit Message Format

We follow Conventional Commits:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Tests
- `chore`: Maintenance

**Example:**
```
feat(wallet): add hardware wallet support

Implemented Ledger Nano S/X integration with full transaction signing.

Closes #123
```

---

## PR Process

### Before Submitting

1. ✅ Code compiles without warnings
2. ✅ All tests pass
3. ✅ Code is formatted
4. ✅ Clippy passes
5. ✅ Documentation updated
6. ✅ Changelog entry added (if applicable)

### Creating Pull Request

1. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open PR on GitHub**
   - Go to repository
   - Click "New Pull Request"
   - Select your branch
   - Fill out PR template

3. **PR Template**
   ```markdown
   ## Description
   Brief description of changes
   
   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update
   
   ## Testing
   - [ ] Tests added/updated
   - [ ] Manual testing performed
   
   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Self-review completed
   - [ ] Documentation updated
   - [ ] No new warnings
   ```

### Review Process

1. **Automated Checks**
   - CI builds and tests
   - Security scan
   - Code coverage

2. **Maintainer Review**
   - At least 1 approval required
   - Address review comments
   - Request re-review after changes

3. **Merge**
   - Squash and merge preferred
   - Merge by maintainer only
   - Delete branch after merge

### After Merge

- Monitor for issues
- Help users with questions
- Update documentation if needed

---

## Additional Resources

- **[API Reference](../API.md)** - Complete API docs
- **[Architecture](../ARCHITECTURE.md)** - System design
- **[Rust Book](https://doc.rust-lang.org/book/)** - Learn Rust
- **[Ethers.rs](https://github.com/gakonst/ethers-rs)** - Ethereum library
- **[WASM Book](https://rustwasm.github.io/docs/book/)** - WebAssembly guide

## Getting Help

- 💬 Discord: #dev channel
- 📧 Email: dev@xorion.io
- 🐛 Issues: GitHub Issues
- 💡 Discussions: GitHub Discussions

---

**Last Updated**: April 2026  
**Version**: 1.0.0

Happy coding! 🚀
