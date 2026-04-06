# Contributing to Xorion Web3 SDK

Thanks for your interest in contributing! This guide covers setup, testing, and the PR process.

## Setup

1. **Fork and clone** the repository:

```bash
git clone https://github.com/<your-username>/xorion-web3-os.git
cd xorion-web3-os
```

2. **Install prerequisites**:

- Rust 1.70+ via [rustup](https://rustup.rs/)
- System libraries (Linux):
  ```bash
  sudo apt-get install -y build-essential pkg-config libssl-dev \
    libgtk-3-dev libwebkit2gtk-4.1-dev libxdo-dev
  ```

3. **Build the workspace**:

```bash
cargo build --workspace
```

## Running Tests

```bash
# Run all tests (229 tests across 7 crates)
cargo test --workspace

# Run tests for a specific crate
cargo test -p xorion-sdk
cargo test -p xorion-core
cargo test -p xorion-governance
cargo test -p xorion-ipfs
cargo test -p xorion-zk

# Run integration tests only
cargo test --test integration

# Run with output
cargo test --workspace -- --nocapture
```

## Linting

All code must pass clippy with warnings denied:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

## Code Style

- Follow standard Rust conventions (`rustfmt` defaults)
- Format before committing:
  ```bash
  cargo fmt --all
  ```
- No unnecessary `unsafe` blocks
- Use `thiserror` for error types
- Prefer `Result<T, E>` over panics

## Pull Request Process

1. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make your changes** and ensure:
   - `cargo test --workspace` passes
   - `cargo clippy --workspace -- -D warnings` is clean
   - `cargo fmt --all --check` has no diffs

3. **Commit** with a clear message:
   ```
   feat: add support for Polygon RPC
   fix: handle empty mnemonic in wallet creation
   docs: update IPFS storage examples
   refactor: simplify proposal state transitions
   ```

4. **Push and open a PR** against `main`:
   ```bash
   git push -u origin feature/my-feature
   ```

5. **CI must pass** — the PR will be checked automatically for tests, clippy, and documentation builds.

## Crate Overview

| Directory | Crate | What it does |
|-----------|-------|-------------|
| `src/` | `xorion-sdk` | Wallet, RPC, contracts |
| `xorion-runtime/` | `xorion-core` | WASM sandbox runtime |
| `xorion-privacy/` | `xorion-zk` | zk-SNARKs proofs |
| `xorion-storage/` | `xorion-ipfs` | IPFS + encryption |
| `xorion-governance/` | `xorion-governance` | DAO governance |
| `xorion-scheme/` | `xorion-scheme` | Redox OS daemon |
| `xorion-gui/` | `xorion-gui` | Desktop GUI |

## Reporting Issues

Open an issue at [GitHub Issues](https://github.com/IDevNation/xorion-web3-os/issues) with:
- Steps to reproduce
- Expected vs actual behavior
- Rust version (`rustc --version`)
- OS and version

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
