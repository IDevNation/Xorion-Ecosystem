//! Client-side encryption using AES-256-GCM with Argon2 key derivation.
//!
//! - Password -> 32-byte key via Argon2id
//! - Encrypt: AES-256-GCM with random 12-byte nonce (prepended to ciphertext)
//! - Decrypt: split nonce, decrypt remaining ciphertext
//! - Key is zeroized on drop

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::Aes256Gcm;
use rand::RngCore;
use zeroize::Zeroize;

use crate::{Result, StorageError};

/// AES-256-GCM encryption with Argon2-derived keys.
pub struct Encryption {
    key: [u8; 32],
}

impl Encryption {
    /// Derive a 256-bit key from a password and salt using Argon2id.
    ///
    /// Salt must be at least 8 bytes.
    pub fn from_password(password: &str, salt: &[u8]) -> Result<Self> {
        if salt.len() < 8 {
            return Err(StorageError::Encryption(
                "salt must be at least 8 bytes".into(),
            ));
        }

        let argon2 = argon2::Argon2::default();
        let mut key = [0u8; 32];

        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| StorageError::Encryption(format!("key derivation failed: {e}")))?;

        Ok(Self { key })
    }

    /// Create encryption from a raw 32-byte key (for testing or pre-derived keys).
    pub fn from_raw_key(key: [u8; 32]) -> Self {
        Self { key }
    }

    /// Encrypt plaintext. Returns nonce (12 bytes) || ciphertext || tag.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(&self.key.into());

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = aes_gcm::Nonce::from(nonce_bytes);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| StorageError::Encryption(format!("encrypt failed: {e}")))?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt data produced by [`encrypt`]. Expects nonce || ciphertext || tag.
    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>> {
        if encrypted.len() < 12 + 16 {
            // 12 nonce + 16 tag minimum
            return Err(StorageError::Encryption("data too short".into()));
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
        let cipher = Aes256Gcm::new(&self.key.into());

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| StorageError::Encryption(format!("decrypt failed: {e}")))
    }
}

impl Drop for Encryption {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SALT: &[u8] = b"xorion_test_salt_16bytes";
    const TEST_PASSWORD: &str = "hunter2";

    #[test]
    fn roundtrip() {
        let enc = Encryption::from_password(TEST_PASSWORD, TEST_SALT).unwrap();
        let plaintext = b"Hello, Xorion!";
        let encrypted = enc.encrypt(plaintext).unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();
        assert_eq!(&decrypted, plaintext);
    }

    #[test]
    fn empty_plaintext_roundtrip() {
        let enc = Encryption::from_password(TEST_PASSWORD, TEST_SALT).unwrap();
        let encrypted = enc.encrypt(b"").unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn large_data_roundtrip() {
        let enc = Encryption::from_password(TEST_PASSWORD, TEST_SALT).unwrap();
        let data = vec![0xABu8; 1_000_000]; // 1 MB
        let encrypted = enc.encrypt(&data).unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn wrong_password_fails() {
        let enc1 = Encryption::from_password("correct", TEST_SALT).unwrap();
        let enc2 = Encryption::from_password("wrong", TEST_SALT).unwrap();
        let encrypted = enc1.encrypt(b"secret").unwrap();
        assert!(enc2.decrypt(&encrypted).is_err());
    }

    #[test]
    fn nonce_prepended() {
        let enc = Encryption::from_password(TEST_PASSWORD, TEST_SALT).unwrap();
        let encrypted = enc.encrypt(b"data").unwrap();
        // 12 nonce + 4 plaintext + 16 GCM tag = 32 bytes
        assert_eq!(encrypted.len(), 12 + 4 + 16);
    }

    #[test]
    fn different_encryptions_differ() {
        let enc = Encryption::from_password(TEST_PASSWORD, TEST_SALT).unwrap();
        let e1 = enc.encrypt(b"same").unwrap();
        let e2 = enc.encrypt(b"same").unwrap();
        // Random nonce means ciphertexts differ
        assert_ne!(e1, e2);
    }

    #[test]
    fn short_salt_rejected() {
        let result = Encryption::from_password("pass", b"short");
        assert!(result.is_err());
    }

    #[test]
    fn truncated_ciphertext_rejected() {
        let result = Encryption::from_raw_key([0u8; 32]).decrypt(&[0u8; 10]);
        assert!(result.is_err());
    }

    #[test]
    fn from_raw_key_works() {
        let key = [42u8; 32];
        let enc = Encryption::from_raw_key(key);
        let encrypted = enc.encrypt(b"test").unwrap();
        let enc2 = Encryption::from_raw_key(key);
        let decrypted = enc2.decrypt(&encrypted).unwrap();
        assert_eq!(&decrypted, b"test");
    }
}
