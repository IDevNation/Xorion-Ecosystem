# X-OS Storage (xos-storage)

[![Crates.io](https://img.shields.io/crates/v/xorion-storage.svg)](https://crates.io/crates/xorion-storage)
[![Documentation](https://docs.rs/xorion-storage/badge.svg)](https://docs.rs/xorion-storage)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

X-OS Storage provides decentralized, encrypted storage built on IPFS with advanced encryption, virtual filesystem abstraction, and automatic pinning for the Xorion ecosystem.

## Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🔐 Encryption | AES-256-GCM with Argon2 KDF | ✅ Complete |
| 📁 Virtual FS | POSIX-like filesystem interface | ✅ Complete |
| 📌 Auto-Pinning | Automatic IPFS pinning service | ✅ Complete |
| 🌐 Gateway Access | Multiple IPFS gateway support | ✅ Complete |
| 🔄 Sync | Bidirectional sync with IPFS | ✅ Complete |
| 📊 Metadata | Rich metadata and indexing | ✅ Complete |

## Installation

\\\	oml
[dependencies]
xorion-storage = "2.0"
ipfs-api-backend-hyper = "0.6"
aes-gcm = "0.10"
argon2 = "0.5"
tokio = { version = "1.35", features = ["full"] }
\\\

## Quick Start

\\\ust
use xorion_storage::{IpfsStorage, EncryptionConfig};
use std::path::Path;

// Initialize storage
let config = EncryptionConfig::new()
    .password("your-secret-password")
    .salt(random_salt());
    
let mut storage = IpfsStorage::with_encryption(config).await?;

// Upload file
let cid = storage.upload(Path::new("./document.pdf")).await?;
println!("Uploaded: {}", cid);

// Download file
storage.download(&cid, Path::new("./downloaded.pdf")).await?;

// List files
let files = storage.list("/my-folder").await?;
for file in files {
    println!("{} - {} bytes", file.name, file.size);
}
\\\

## Encryption Details

### Algorithm
- **Cipher**: AES-256-GCM
- **KDF**: Argon2id (memory-hard)
- **Key Size**: 256 bits
- **Nonce**: 96 bits (random per file)
- **Tag**: 128 bits authentication

### Key Derivation
\\\ust
let kdf_config = argon2::Config {
    time_cost: 3,
    mem_cost: 65536, // 64 MB
    parallelism: 4,
    hash_length: 32,
    variant: argon2::Variant::Argon2id,
};
\\\

## Architecture

\\\
┌─────────────────────────────────────────┐
│         X-OS Storage                    │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   Virtual   │  │   Encryption     │  │
│  │   Filesystem│  │   Layer          │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   IPFS      │  │   Pinning        │  │
│  │   Client    │  │   Service        │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │   Gateway   │  │   Metadata       │  │
│  │   Router    │  │   Indexer        │  │
│  └─────────────┘  └──────────────────┘  │
└─────────────────────────────────────────┘
         ▲
         │
    IPFS Network
\\\

## IPFS Gateways

| Gateway | URL | Type |
|---------|-----|------|
| Local | http://localhost:8080 | Self-hosted |
| Cloudflare | https://cloudflare-ipfs.com | Public |
| Infura | https://ipfs.infura.io | Managed |
| Pinata | https://gateway.pinata.cloud | Managed |

## Performance Metrics

| Operation | Latency | Throughput |
|-----------|---------|------------|
| Upload (1MB) | ~500ms | ~2 MB/s |
| Download (1MB) | ~300ms | ~3 MB/s |
| Encryption | ~50ms | ~20 MB/s |
| Pinning | ~200ms | N/A |

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
| ipfs-api-backend-hyper | 0.6 | IPFS HTTP client |
| aes-gcm | 0.10 | Authenticated encryption |
| argon2 | 0.5 | Password hashing |
| tokio | 1.35 | Async runtime |
| serde | 1.0 | Serialization |

## Security Considerations

- **End-to-End Encryption**: Files encrypted before leaving client
- **Zero-Knowledge**: Server cannot decrypt content
- **Key Management**: Keys derived from user password
- **Integrity**: HMAC verification on download

## Troubleshooting

**Issue**: Connection refused to IPFS node  
**Solution**: Ensure IPFS daemon is running: \ipfs daemon\

**Issue**: Decryption failed  
**Solution**: Verify correct password and salt used

## License

MIT License - see [LICENSE](../LICENSE) for details.

## Links

- [Main Documentation](../README.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Change Log](../CHANGELOG.md)
