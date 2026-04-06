use bip32::{DerivationPath, XPrv};
use bip39::Mnemonic;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::fmt;
use std::str::FromStr;
use zeroize::Zeroize;

use log::{debug, info};

use crate::error::{Result, WalletError};

/// Derivation path for Ethereum: m/44'/60'/0'/0/0
const ETH_DERIVATION_PATH: &str = "m/44'/60'/0'/0/0";

/// Derivation path for Solana: m/44'/501'/0'/0'
const SOL_DERIVATION_PATH: &str = "m/44'/501'/0'/0'";

/// A multi-chain HD wallet derived from a BIP-39 mnemonic.
pub struct Wallet {
    seed: Vec<u8>,
    mnemonic_phrase: String,
}

impl Wallet {
    /// Create a new wallet from a BIP-39 mnemonic phrase (12 or 24 words).
    pub fn from_mnemonic(phrase: &str) -> Result<Self> {
        let mnemonic = Mnemonic::parse(phrase)
            .map_err(|e| WalletError::InvalidMnemonic(e.to_string()))?;

        let seed = mnemonic.to_seed("");
        if seed.len() < 16 {
            return Err(WalletError::InvalidSeed(
                "seed is too short".to_string(),
            ));
        }

        info!("wallet created from mnemonic");
        Ok(Self {
            seed: seed.to_vec(),
            mnemonic_phrase: phrase.to_string(),
        })
    }

    /// Derive an Ethereum address from the standard BIP-44 path m/44'/60'/0'/0/0.
    ///
    /// Returns the checksummed address as a hex string prefixed with "0x".
    pub fn derive_eth_address(&self) -> Result<String> {
        let path = DerivationPath::from_str(ETH_DERIVATION_PATH)
            .map_err(|e| WalletError::DerivationError(e.to_string()))?;

        let child_xprv = XPrv::derive_from_path(&self.seed, &path)
            .map_err(|e| WalletError::DerivationError(e.to_string()))?;

        let secret_key = SecretKey::from_slice(&child_xprv.to_bytes())
            .map_err(|e| WalletError::CryptoError(e.to_string()))?;

        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        // Ethereum uses the uncompressed public key without the 0x04 prefix
        let uncompressed = public_key.serialize_uncompressed();
        let pub_key_bytes = &uncompressed[1..]; // skip the 0x04 prefix byte

        let mut hasher = Keccak256::new();
        hasher.update(pub_key_bytes);
        let hash = hasher.finalize();

        // Ethereum address is the last 20 bytes of the Keccak-256 hash
        let raw_address = &hash[12..];
        let address_hex = hex::encode(raw_address);

        // EIP-55 checksum encoding
        let checksummed = eip55_checksum(&address_hex);

        debug!("derived ETH address: 0x{checksummed}");
        Ok(format!("0x{checksummed}"))
    }

    /// Derive a Solana address from the standard BIP-44 path m/44'/501'/0'/0'.
    ///
    /// Returns the address as a base-58 encoded string (using hex here since
    /// we don't pull in a base58 crate — the raw 32-byte public key is shown in hex).
    pub fn derive_solana_address(&self) -> Result<String> {
        let path = DerivationPath::from_str(SOL_DERIVATION_PATH)
            .map_err(|e| WalletError::DerivationError(e.to_string()))?;

        let child_xprv = XPrv::derive_from_path(&self.seed, &path)
            .map_err(|e| WalletError::DerivationError(e.to_string()))?;

        let secret_key = SecretKey::from_slice(&child_xprv.to_bytes())
            .map_err(|e| WalletError::CryptoError(e.to_string()))?;

        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        // For Solana we use the compressed 33-byte secp256k1 public key (hex-encoded).
        // A production SDK would use ed25519 and base58 — this demonstrates
        // the HD derivation pipeline with the secp256k1 stack.
        let compressed = public_key.serialize();
        let addr = hex::encode(compressed);
        debug!("derived SOL address: {addr}");
        Ok(addr)
    }

    /// Return the mnemonic phrase (handle with care — sensitive material).
    pub fn mnemonic(&self) -> &str {
        &self.mnemonic_phrase
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let eth = self
            .derive_eth_address()
            .unwrap_or_else(|e| format!("<error: {e}>"));
        let sol = self
            .derive_solana_address()
            .unwrap_or_else(|e| format!("<error: {e}>"));

        writeln!(f, "=== Xorion Multi-Chain Wallet ===")?;
        writeln!(f, "Ethereum : {eth}")?;
        writeln!(f, "Solana   : {sol}")
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        self.seed.zeroize();
        // Safety: zeroize the mnemonic in memory on drop
        // SAFETY: we own the string; zeroing its bytes is safe before it is dropped.
        unsafe {
            let bytes = self.mnemonic_phrase.as_bytes_mut();
            bytes.zeroize();
        }
    }
}

/// Apply EIP-55 mixed-case checksum to a lowercase hex address (without 0x prefix).
fn eip55_checksum(address: &str) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(address.as_bytes());
    let hash = hex::encode(hasher.finalize());

    address
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_ascii_alphabetic() {
                let nibble = u8::from_str_radix(&hash[i..i + 1], 16).unwrap_or(0);
                if nibble >= 8 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Standard BIP-39 test vector mnemonic
    const TEST_MNEMONIC: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn wallet_from_valid_mnemonic() {
        let wallet = Wallet::from_mnemonic(TEST_MNEMONIC);
        assert!(wallet.is_ok());
    }

    #[test]
    fn wallet_from_invalid_mnemonic() {
        let wallet = Wallet::from_mnemonic("invalid mnemonic phrase");
        assert!(wallet.is_err());
    }

    #[test]
    fn derive_eth_address_succeeds() {
        let wallet = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let addr = wallet.derive_eth_address().unwrap();
        assert!(addr.starts_with("0x"));
        // 0x + 40 hex chars = 42
        assert_eq!(addr.len(), 42);
    }

    #[test]
    fn derive_solana_address_succeeds() {
        let wallet = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let addr = wallet.derive_solana_address().unwrap();
        assert!(!addr.is_empty());
    }

    #[test]
    fn display_shows_both_chains() {
        let wallet = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let output = format!("{wallet}");
        assert!(output.contains("Ethereum"));
        assert!(output.contains("Solana"));
    }
}
