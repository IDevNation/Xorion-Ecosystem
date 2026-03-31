//! Xorion Decentralized Storage — IPFS-backed filesystem with client-side encryption
//!
//! Provides:
//! - **IPFS client** — upload, download, pin files via the IPFS HTTP API
//! - **AES-256-GCM encryption** — client-side encrypt/decrypt with Argon2 key derivation
//! - **Virtual filesystem** — directory tree mapped to IPFS CIDs
//! - **Pinning service** — track and manage pinned content
//! - **Local cache** — LRU-style disk cache with configurable size limits

pub mod cache;
pub mod encryption;
pub mod ipfs;
pub mod pinning;
pub mod vfs;

pub use cache::FileCache;
pub use encryption::Encryption;
pub use ipfs::IpfsClient;
pub use pinning::PinningService;
pub use vfs::VirtualFs;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IPFS error: {0}")]
    Ipfs(String),

    #[error("encryption error: {0}")]
    Encryption(String),

    #[error("filesystem error: {0}")]
    Filesystem(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("cache error: {0}")]
    Cache(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;
