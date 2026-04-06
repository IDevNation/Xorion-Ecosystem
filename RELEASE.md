# X-OS v1.0.0-beta

**Release Date:** March 2026
**Status:** Beta
**Tests:** 230+ passing

---

## What is Xorion?

Xorion is a **Web3-native operating system** built entirely in Rust. It provides blockchain primitives at the OS level - wallets, RPC, smart contracts, WASM execution, zero-knowledge proofs, decentralized storage, and DAO governance - all in a single integrated workspace.

---

## Features

### Core SDK (`xorion-wallet-sdk`)
- BIP-39/BIP-44 HD wallet with Ethereum + Solana support
- EIP-55 checksum addresses, zeroize-on-drop key material
- Async JSON-RPC providers for ETH and SOL
- ABI encoding/decoding, ERC-20 interface, Uniswap V2 Router/Pair
- Redox OS `wallet:` scheme daemon with per-process isolation

### Desktop GUI (`xorion-gui`)
- Dioxus-based dark-themed wallet application
- Dashboard, Send, Receive (QR), Settings views
- Connects to all SDK features

### WASM Runtime (`xorion-runtime`)
- Wasmtime 19 sandboxed execution engine
- Permission-based isolation (ReadWallet, SignTransaction, Network, Storage)
- Host wallet bridge functions exposed to WASM guests
- IPFS dApp loading

### ZK Privacy (`xorion-privacy`)
- Groth16 zk-SNARKs on BN254 curve (arkworks)
- Private transaction proofs (hide balance/amount)
- Age verification (prove age >= N without revealing DOB)
- Balance proofs (prove sufficient funds without revealing amount)
- Proof caching with configurable TTL

### IPFS Storage (`xorion-storage`)
- IPFS HTTP API client (upload, download, pin, list)
- AES-256-GCM client-side encryption with Argon2id key derivation
- Virtual filesystem mapped to IPFS CIDs
- Pin management with metadata tracking
- Disk-backed LRU cache with size-based eviction

### DAO Governance (`xorion-governance`)
- Proposal lifecycle: Pending -> Active -> Succeeded/Defeated -> Queued -> Executed
- Token-weighted voting (For/Against/Abstain) with quorum enforcement
- Voting power delegation
- Treasury management with audit log
- Timelock execution delay
- Governor contract ABI encoding (OpenZeppelin compatible)

---

## Installation

### From Source

```bash
# Prerequisites: Rust 1.70+, Git
# Linux GUI deps: sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev libxdo-dev

git clone https://github.com/IDevNation/xorion-web3-os.git
cd xorion-web3-os
cargo build --release --workspace
cargo test --workspace
```

### Run Examples

```bash
cargo run --example demo                              # Wallet creation
cargo run --example rpc_demo                           # RPC integration
cargo run --example contract_demo                      # Smart contracts
cargo run -p xorion-runtime --example simple_dapp      # WASM runtime
cargo run -p xorion-privacy --example privacy_demo     # ZK proofs
cargo run -p xorion-storage --example storage_demo     # IPFS storage
cargo run -p xorion-governance --example governance_demo # DAO governance
```

### Launch Desktop GUI

```bash
cargo run -p xorion-gui
```

---

## Workspace Crates

| Crate | Version | Description |
|-------|---------|-------------|
| `xorion-wallet-sdk` | 0.4.0 | Core multi-chain wallet SDK |
| `xorion-scheme` | 0.1.0 | Redox OS scheme daemon |
| `xorion-gui` | 0.1.0 | Desktop GUI (Dioxus) |
| `xorion-runtime` | 0.1.0 | WASM dApp runtime (Wasmtime) |
| `xorion-privacy` | 0.1.0 | zk-SNARKs privacy layer (Groth16) |
| `xorion-storage` | 0.1.0 | IPFS storage + encryption |
| `xorion-governance` | 0.1.0 | DAO governance module |

---

## Known Issues

1. **GUI requires Linux system libraries** — GTK3, WebKit2GTK 4.1, libxdo must be installed separately on Linux. macOS uses native WebView.

2. **IPFS client requires running node** — The `IpfsClient` needs a local IPFS node (`http://127.0.0.1:5001`) for upload/pin operations. Read-only fetches fall back to the public gateway.

3. **ZK proof generation is CPU-intensive** — Groth16 trusted setup and proving are computationally expensive. Use `ProofCache` to avoid redundant re-computation.

4. **Redox scheme daemon is Redox-only** — The `wallet:` scheme daemon targets Redox OS. On Linux/macOS/Windows, use the `WalletClient` API directly.

5. **No hardware wallet support yet** — Currently software-only key management. Hardware wallet (Ledger/Trezor) integration is planned for v1.1.

---

## Security

### Audit Status: **Pending**

This is a beta release. A formal security audit has not yet been completed.

### Security Measures in Place

- Private keys zeroized on drop (`zeroize` crate)
- AES-256-GCM encryption for stored data
- Argon2id key derivation (memory-hard, resistant to GPU/ASIC attacks)
- WASM sandbox with permission-based isolation
- Per-process isolation in Redox scheme daemon
- No private keys in kernel space (userspace architecture)

### Responsible Disclosure

If you discover a security vulnerability, please email security@idevnation.com before creating a public issue.

---

## System Requirements

| Component | Requirement |
|-----------|-------------|
| Rust | 1.70+ (stable) |
| OS | Linux (x86_64), macOS (x86_64/arm64), Windows (x86_64) |
| GUI | GTK3 + WebKit2GTK 4.1 + libxdo (Linux only) |
| IPFS | Local node for upload/pin (optional, gateway fallback for reads) |
| Redox | Redox OS for scheme daemon (optional) |

---

## What's Next (v1.1 Roadmap)

- Hardware wallet integration (Ledger, Trezor)
- Mobile app (iOS/Android via Dioxus mobile)
- Layer 2 support (Arbitrum, Optimism, Base)
- NFT marketplace integration
- Cross-chain bridge protocol
- Formal security audit

---

## License

MIT License - see [LICENSE](LICENSE)

---

**Made with Rust for the decentralized web.**
