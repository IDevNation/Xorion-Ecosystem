# Installation Guide

## Prerequisites

- **Rust 1.70+** — Install via [rustup](https://rustup.rs/)
- **Git** — For cloning the repository
- **pkg-config** — Required for some native dependencies
- **OpenSSL dev headers** — Required by reqwest

### Linux (Ubuntu/Debian)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install system dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# For GUI support (optional)
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libxdo-dev
```

### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install system dependencies
brew install openssl pkg-config
```

### Windows

```powershell
# Install Rust from https://rustup.rs/
# Visual Studio Build Tools are required (installed automatically by rustup)
```

## Install from crates.io

Add any combination of Xorion crates to your `Cargo.toml`:

```toml
[dependencies]
xorion-sdk = "0.4.0"          # Core: wallet, RPC, contracts
xorion-core = "0.1.0"         # WASM dApp runtime
xorion-governance = "0.1.0"   # DAO governance module
xorion-ipfs = "0.1.0"         # IPFS storage + encryption
xorion-zk = "0.1.0"           # zk-SNARKs privacy layer
```

## Build from Source

```bash
git clone https://github.com/IDevNation/xorion-web3-os.git
cd xorion-web3-os

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Build in release mode
cargo build --workspace --release
```

## Verify Installation

```bash
# Should compile and run successfully
cargo run --example demo
```

Expected output:
```
=== Xorion Wallet Demo ===
Mnemonic: abandon abandon abandon ...
ETH Address: 0x...
SOL Address: ...
```

## Troubleshooting

### `glib-2.0 not found`
This happens when building the GUI crate without GTK libraries:
```bash
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev
```

### `openssl-sys` build failure
```bash
sudo apt-get install -y libssl-dev    # Debian/Ubuntu
brew install openssl                   # macOS
```

### Slow initial build
The first build compiles ~300 dependencies (arkworks, wasmtime, etc.). Subsequent builds use the cache and are much faster.
