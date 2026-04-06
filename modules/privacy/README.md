# X-OS Privacy (xos-privacy)

[![Crates.io](https://img.shields.io/crates/v/xorion-privacy.svg)](https://crates.io/crates/xorion-privacy)
[![Documentation](https://docs.rs/xorion-privacy/badge.svg)](https://docs.rs/xorion-privacy)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

X-OS Privacy provides zero-knowledge proof capabilities using zk-SNARKs for private transactions and confidential computations within the Xorion ecosystem, built on the arkworks cryptography framework.

## Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🔒 zk-SNARKs | Groth16 proving system | ✅ Complete |
| 🎯 BN254 Curve | Optimized elliptic curve | ✅ Complete |
| 🧩 Circuit Builder | DSL for ZK circuits | ✅ Complete |
| 📝 Trusted Setup | MPC ceremony support | ✅ Complete |
| 🔄 Private Transfers | Confidential token transfers | ✅ Complete |
| 📊 Proof Verification | Fast on-chain verification | ✅ Complete |

## Installation

\\\	oml
[dependencies]
xorion-privacy = "2.0"
arkworks-rs = "0.4"
ark-bn254 = "0.4"
ark-groth16 = "0.4"
ark-ff = "0.4"
tokio = { version = "1.35", features = ["full"] }
\\\

## Quick Start

\\\ust
use xorion_privacy::{ZkProver, Circuit, Proof};
use ark_bn254::Bn254;

// Initialize prover
let mut prover = ZkProver::<Bn254>::new();

// Define circuit
let circuit = Circuit::new()
    .private_input(secret_value)
    .public_input(public_hash)
    .constraint(|witness| witness.secret * witness.public == witness.result);

// Generate proof
let proof: Proof = prover.prove(circuit).await?;

// Verify proof
let valid = prover.verify(&proof, public_inputs).await?;
println!("Proof valid: {}", valid);
\\\

## Supported Curves

| Curve | Security | Performance | Use Case |
|-------|----------|-------------|----------|
| BN254 | 128-bit | Fastest | General purpose |
| BLS12-381 | 128-bit | Fast | Ethereum compatible |
| BW6-761 | 192-bit | Medium | High security |

## Circuit Examples

### Private Transfer
\\\ust
let circuit = PrivateTransferCircuit {
    sender_balance: private!(2500),
    receiver_balance: private!(1000),
    amount: private!(500),
    new_sender_balance: public!(2000),
    new_receiver_balance: public!(1500),
};
\\\

### Range Proof
\\\ust
let circuit = RangeProofCircuit {
    value: private!(age),
    min: public!(18),
    max: public!(100),
};
\\\

## Trusted Setup

### Phase 1 (Powers of Tau)
\\\ash
cargo run --bin setup -- phase1 --powers 14
\\\

### Phase 2 (Circuit-specific)
\\\ash
cargo run --bin setup -- phase2 --circuit transfer
\\\

### Verify Transcript
\\\ash
cargo run --bin setup -- verify
\\\

## Architecture

\\\
┌─────────────────────────────────────────┐
│         X-OS Privacy                    │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   Circuit   │  │   Prover         │  │
│  │   Builder   │  │   Engine         │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   Verifier  │  │   Trusted        │  │
│  │   Module    │  │   Setup          │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   Crypto    │  │   Proof          │  │
│  │   Primitives│  │   Serializer     │  │
│  └─────────────┘  └──────────────────┘  │
└─────────────────────────────────────────┘
         ▲
         │
    Blockchain
\\\

## Performance Benchmarks

| Operation | Time | Size |
|-----------|------|------|
| Proof Generation | 2-5s | 288 bytes |
| Verification | 10-50ms | 288 bytes |
| Circuit Compilation | 100-500ms | N/A |
| Trusted Setup | 10-30min | 100MB+ |

## Security Notes

- **Trusted Setup**: Requires secure MPC ceremony
- **Soundness**: Computational soundness under DLOG assumption
- **Zero-Knowledge**: Perfect zero-knowledge property
- **Succinctness**: Constant-size proofs

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

# Benchmark
cargo bench
\\\

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| ark-bn254 | 0.4 | BN254 curve implementation |
| ark-groth16 | 0.4 | Groth16 proving system |
| ark-ff | 0.4 | Finite field arithmetic |
| ark-std | 0.4 | Standard utilities |
| rand | 0.8 | Random number generation |

## Troubleshooting

**Issue**: Out of memory during proof generation  
**Solution**: Reduce circuit complexity or increase RAM

**Issue**: Proof verification fails  
**Solution**: Ensure matching trusted setup parameters

## License

MIT License - see [LICENSE](../LICENSE) for details.

## Links

- [Main Documentation](../README.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Change Log](../CHANGELOG.md)
