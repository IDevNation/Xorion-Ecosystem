//! # Xorion IPFS — Decentralized Storage
//!
//! IPFS-backed filesystem with client-side encryption:
//! - **IPFS client** — upload, download, pin files via the IPFS HTTP API
//! - **AES-256-GCM encryption** — client-side encrypt/decrypt with Argon2 key derivation
//! - **Virtual filesystem** — directory tree mapped to IPFS CIDs
//! - **Pinning service** — track and manage pinned content
//! - **Local cache** — LRU-style disk cache with configurable size limits
//!
//! ## Example
//!
//! ```rust
//! use xos_storage::Encryption;
//!
//! let enc = Encryption::from_password("secret", b"salt_at_least_8b").unwrap();
//! let ciphertext = enc.encrypt(b"hello world").unwrap();
//! let plaintext = enc.decrypt(&ciphertext).unwrap();
//! assert_eq!(plaintext, b"hello world");
//! ```

pub mod cache;
pub mod encryption;
pub mod ipfs;
pub mod pinning;
pub mod vfs;

/// Disk-backed LRU file cache with configurable size limits.
pub use cache::FileCache;
/// AES-256-GCM encryption with Argon2id key derivation.
pub use encryption::Encryption;
/// Async IPFS HTTP API client for add, cat, and pin operations.
pub use ipfs::IpfsClient;
/// Pin management service for tracking pinned IPFS content.
pub use pinning::PinningService;
/// Virtual filesystem that maps paths to IPFS CIDs.
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
