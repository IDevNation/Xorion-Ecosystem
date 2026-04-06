# X-OS Runtime (xos-runtime)

[![Crates.io](https://img.shields.io/crates/v/xorion-sdk.svg)](https://crates.io/crates/xorion-sdk)
[![Documentation](https://docs.rs/xorion-sdk/badge.svg)](https://docs.rs/xorion-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

X-OS Runtime is the WebAssembly (WASM) execution engine for decentralized applications (dApps) within the Xorion ecosystem. It provides a secure, sandboxed environment for running smart contracts and dApp logic with full WASI support.

## Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🚀 WASM Execution | High-performance Wasmer-based runtime | ✅ Complete |
| 🔒 Sandboxing | Secure isolation for untrusted code | ✅ Complete |
| 🌐 WASI Support | Full WebAssembly System Interface | ✅ Complete |
| 📦 IPFS Loading | Load dApps directly from IPFS | ✅ Complete |
| 🔌 Host Functions | Custom host function integration | ✅ Complete |
| 📊 Resource Limits | Configurable CPU/memory limits | ✅ Complete |

## Installation

\\\	oml
[dependencies]
xorion-sdk = "2.0"
wasmer = "4.3"
wasmer-wasi = "4.3"
\\\

## Quick Start

\\\ust
use xorion_sdk::runtime::XosRuntime;
use xorion_sdk::storage::IpfsStorage;

// Initialize runtime
let mut runtime = XosRuntime::new();

// Load dApp from IPFS
let wasm_bytes = IpfsStorage::get("QmYourDAppHash").await?;

// Execute dApp
let result = runtime.execute(&wasm_bytes, &input_data)?;
println!("Result: {:?}", result);
\\\

## Architecture

\\\
┌─────────────────────────────────────────┐
│           X-OS Runtime                  │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   Wasmer    │  │   WASI Layer     │  │
│  │   Engine    │  │   (POSIX-like)   │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   Host      │  │   Resource       │  │
│  │   Functions │  │   Manager        │  │
│  └─────────────┘  └──────────────────┘  │
└─────────────────────────────────────────┘
         ▲                       ▲
         │                       │
    IPFS Storage          Security Layer
\\\

## API Reference

### XosRuntime

\\\ust
pub struct XosRuntime {
    store: Store,
    instance: Instance,
    limits: ResourceLimits,
}

impl XosRuntime {
    pub fn new() -> Self;
    pub fn load_wasm(&mut self, wasm_bytes: &[u8]) -> Result<()>;
    pub fn execute(&mut self, method: &str, input: &[u8]) -> Result<Vec<u8>>;
    pub fn set_limits(&mut self, cpu: u64, memory: u64);
}
\\\

## Security Considerations

- **Sandboxing**: All dApps run in isolated WASM sandboxes
- **Resource Limits**: Configurable CPU and memory constraints
- **No Direct FS Access**: Filesystem access only through WASI
- **Validation**: All WASM modules are validated before execution

## Development

\\\ash
# Build
cargo build --release

# Test
cargo test

# Lint
cargo clippy

# Generate docs
cargo doc --open
\\\

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| wasmer | 4.3 | WASM runtime engine |
| wasmer-wasi | 4.3 | WASI implementation |
| xorion-core | 2.0 | Core utilities |
| tokio | 1.35 | Async runtime |

## Troubleshooting

**Issue**: WASM module fails to load  
**Solution**: Ensure module is compiled with --target wasm32-wasi

**Issue**: Out of memory errors  
**Solution**: Increase memory limits via set_limits()

## License

MIT License - see [LICENSE](../LICENSE) for details.

## Links

- [Main Documentation](../README.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Change Log](../CHANGELOG.md)
