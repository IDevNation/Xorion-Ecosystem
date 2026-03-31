# Xorion Web3 SDK

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Status](https://img.shields.io/badge/status-beta-blue.svg)
![Tests](https://img.shields.io/badge/tests-229%20passing-brightgreen.svg)
![Version](https://img.shields.io/badge/version-v1.0.0--beta.1-blue.svg)
![Release](https://img.shields.io/github/v/release/IDevNation/xorion-web3-os?include_prereleases&label=release)

> **The Web3-Native Operating System** — Built in Rust. Web3 at the kernel level.

---

## 🎯 Vision

Xorion is a decentralized operating system where blockchain primitives live at the kernel level — not browser extensions. One OS to rule all chains (Ethereum + Solana).

---

## ✨ Core Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🔐 **Multi-Chain Wallet** | BIP39/BIP44 HD wallet (Ethereum + Solana) | ✅ Complete |
| ⛓️ **Native Blockchain SDK** | Direct RPC integration (ETH + SOL) | ✅ Complete |
| 📋 **Smart Contracts** | ABI parsing, function calls, events | ✅ Complete |
| 💰 **Token Standards** | ERC20, ERC721 (NFTs) support | ✅ Complete |
| 🔄 **DeFi Protocols** | Uniswap, Aave integration | ✅ Complete |
| 🏗️ **Redox Scheme** | wallet:/ filesystem daemon | ✅ Complete |
| 🎨 **Desktop GUI** | Dioxus-based wallet interface | ✅ Complete |
| 🧩 **WASM Runtime** | Wasmtime sandbox, wallet bridge, IPFS loader | ✅ Complete |
| 🤫 **ZK Privacy** | Groth16 zk-SNARKs — private tx, age verify, balance proofs | ✅ Complete |
| 📦 **IPFS Storage** | IPFS client, AES-256-GCM encryption, virtual FS, pinning, cache | ✅ Complete |

---

## 🗺️ Development Roadmap

| Phase | Description | Status | Tests |
|-------|-------------|--------|-------|
| **Phase 1** | Multi-Chain SDK (Wallet, ETH/SOL addresses) | ✅ Complete | 5 |
| **Phase 2** | RPC Integration & Broadcasting | ✅ Complete | 3 |
| **Phase 3** | Smart Contract Interaction (ERC20, Uniswap) | ✅ Complete | 15 |
| **Phase 4** | Redox Scheme Daemon (wallet:/) | ✅ Complete | 10 |
| **Phase 5** | Desktop GUI (Dioxus) | ✅ Complete | 3 |
| **Phase 6** | WASM dApp Runtime (Wasmtime) | ✅ Complete | 33 |
| **Phase 7** | zk-SNARKs Privacy Layer (Groth16) | ✅ Complete | 38 |
| **Phase 8** | IPFS Native Filesystem | ✅ Complete | 54 |
| **Phase 9** | DAO Governance Module | ✅ Complete | 54 |
| **Phase 10** | Beta Release (v1.0.0-beta) | ✅ Complete | 14 |

**✅ Total Tests: 229/229 Passing**

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              XORION WEB3 OS - FULL STACK                    │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  ┌────────────────────────────────────────────────────┐    │
│  │         DAO Governance — Phase 9                   │    │
│  │    Proposals │ Voting │ Delegation │ Treasury      │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         IPFS Storage — Phase 8                     │    │
│  │    IpfsClient │ Encryption │ VFS │ Pinning │ Cache │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         ZK Privacy (Groth16) — Phase 7             │    │
│  │    PrivateTx │ AgeVerify │ BalanceProof            │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         WASM Runtime (Wasmtime) — Phase 6          │    │
│  │    Sandbox │ WalletBridge │ IPFS Loader            │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         GUI (Dioxus Desktop) — Phase 5             │    │
│  │    Dashboard │ Send │ Receive │ Settings           │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         Redox Scheme (wallet:/) — Phase 4          │    │
│  │    Daemon │ Protocol │ Keyring │ Handler           │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         Xorion SDK — Phases 1-3                    │    │
│  │    Wallet │ RPC │ Contract │ Tokens │ DeFi         │    │
│  └────────────────────────────────────────────────────┘    │
│                           ⬇️                                │
│  ┌────────────────────────────────────────────────────┐    │
│  │         Blockchain Layer                           │    │
│  │    Ethereum (JSON-RPC) │ Solana (JSON-RPC)        │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Installation

### From source

```bash
git clone https://github.com/IDevNation/xorion-web3-os.git
cd xorion-web3-os
cargo build --workspace
```

### As a dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
xorion-wallet-sdk = { git = "https://github.com/IDevNation/xorion-web3-os.git", tag = "v1.0.0-beta.1" }
```

---

## Quick Start

### Prerequisites
- Rust 1.70 or later
- For GUI: `sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libxdo-dev`

### Commands

```bash
# Build entire workspace
cargo build --workspace

# Test (229 tests)
cargo test --workspace

# Run GUI
cargo run -p xorion-gui

# Run WASM runtime demo
cargo run -p xorion-runtime --example simple_dapp

# Run ZK privacy demo
cargo run -p xorion-privacy --example privacy_demo

# Run IPFS storage demo
cargo run -p xorion-storage --example storage_demo

# Run DAO governance demo
cargo run -p xorion-governance --example governance_demo

# Run examples
cargo run --example demo              # Wallet creation
cargo run --example rpc_demo          # RPC integration
cargo run --example contract_demo     # Smart contracts
cargo run --example kernel_demo       # Full demo
```

### Quick Start Example

```rust
use xorion_sdk::Wallet;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet = Wallet::new("ethereum").await?;
    println!("Address: {}", wallet.get_address());
    Ok(())
}
```

---

## 📁 Project Structure

```
xorion-web3-os/
├── src/                        # Phases 1-3: Core SDK
│   ├── wallet.rs               # BIP39, ETH/SOL addresses
│   ├── rpc/                    # ETH/SOL RPC clients
│   ├── contract/               # ABI, ERC20, Uniswap
│   └── kernel.rs               # WalletClient API
│
├── xorion-scheme/              # Phase 4: Redox Scheme Daemon
│   └── src/
│       ├── main.rs             # wallet:/ scheme
│       ├── protocol.rs         # JSON wire protocol
│       └── handler.rs          # Request handler
│
├── xorion-gui/                 # Phase 5: Desktop GUI
│   └── src/
│       ├── main.rs             # App shell
│       └── components/
│           ├── dashboard.rs
│           ├── send.rs
│           ├── receive.rs
│           └── settings.rs
│
├── xorion-runtime/             # Phase 6: WASM dApp Runtime
│   └── src/
│       ├── runtime.rs          # WasmRuntime (wasmtime engine)
│       ├── sandbox.rs          # Permission-based sandbox
│       ├── api.rs              # WalletBridge host functions
│       └── ipfs_loader.rs      # IPFS dApp fetcher
│
├── xorion-privacy/             # Phase 7: zk-SNARKs Privacy
│   └── src/
│       ├── circuits/
│       │   ├── private_tx.rs   # Private transaction circuit
│       │   ├── age_verify.rs   # Age verification circuit
│       │   └── balance_proof.rs# Balance proof circuit
│       ├── proof.rs            # Groth16 ProofSystem
│       └── cache.rs            # Proof caching with TTL
│
├── xorion-storage/             # Phase 8: IPFS Storage
│   └── src/
│       ├── ipfs.rs             # IPFS HTTP API client
│       ├── encryption.rs       # AES-256-GCM + Argon2
│       ├── vfs.rs              # Virtual filesystem over IPFS
│       ├── pinning.rs          # Pin management
│       └── cache.rs            # Disk-backed LRU cache
│
├── xorion-governance/          # Phase 9: DAO Governance
│   └── src/
│       ├── proposal.rs         # Proposal lifecycle state machine
│       ├── voting.rs           # Token-weighted voting + quorum
│       ├── delegation.rs       # Voting power delegation
│       ├── treasury.rs         # DAO treasury management
│       ├── timelock.rs         # Execution delay enforcement
│       └── governor.rs         # Governor contract ABI encoding
│
├── examples/                   # Demo applications
└── Cargo.toml                  # Workspace
```

---

## 📊 Progress

```
Phase 1: ████████████████████ 100% ✅
Phase 2: ████████████████████ 100% ✅
Phase 3: ████████████████████ 100% ✅
Phase 4: ████████████████████ 100% ✅
Phase 5: ████████████████████ 100% ✅
Phase 6: ████████████████████ 100% ✅
Phase 7: ████████████████████ 100% ✅
Phase 8: ████████████████████ 100% ✅
Phase 9: ████████████████████ 100% ✅
Phase 10:████████████████████ 100% ✅
```

---

## 📄 License

MIT License — see [LICENSE](LICENSE)

---

**Made with ❤️ for the decentralized web** 🌌

*10 phases complete | 229 tests passing | v1.0.0-beta.1 released*
