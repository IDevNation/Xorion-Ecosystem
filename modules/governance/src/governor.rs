//! Governor contract ABI encoding.
//!
//! Encodes function calls for interacting with on-chain OpenZeppelin-style
//! Governor contracts via the Xorion wallet SDK.

use sha3::{Digest, Keccak256};

/// ABI encoder for Governor contract interactions.
pub struct GovernorAbi;

impl GovernorAbi {
    /// Compute a Solidity function selector: first 4 bytes of keccak256(signature).
    pub fn selector(signature: &str) -> [u8; 4] {
        let hash = Keccak256::digest(signature.as_bytes());
        let mut sel = [0u8; 4];
        sel.copy_from_slice(&hash[..4]);
        sel
    }

    /// Encode `propose(address[],uint256[],bytes[],string)` calldata.
    pub fn encode_propose(description: &str) -> Vec<u8> {
        let sel = Self::selector("propose(address[],uint256[],bytes[],string)");
        let mut data = sel.to_vec();
        // Simplified: encode description hash as a 32-byte word
        let desc_hash = Keccak256::digest(description.as_bytes());
        data.extend_from_slice(&desc_hash);
        data
    }

    /// Encode `castVote(uint256,uint8)` calldata.
    /// support: 0 = Against, 1 = For, 2 = Abstain.
    pub fn encode_cast_vote(proposal_id: u64, support: u8) -> Vec<u8> {
        let sel = Self::selector("castVote(uint256,uint8)");
        let mut data = sel.to_vec();
        // proposal_id as uint256 (32 bytes, big-endian)
        let mut id_bytes = [0u8; 32];
        id_bytes[24..32].copy_from_slice(&proposal_id.to_be_bytes());
        data.extend_from_slice(&id_bytes);
        // support as uint8 padded to 32 bytes
        let mut support_bytes = [0u8; 32];
        support_bytes[31] = support;
        data.extend_from_slice(&support_bytes);
        data
    }

    /// Encode `execute(address[],uint256[],bytes[],bytes32)` calldata.
    pub fn encode_execute(description_hash: &[u8; 32]) -> Vec<u8> {
        let sel = Self::selector("execute(address[],uint256[],bytes[],bytes32)");
        let mut data = sel.to_vec();
        data.extend_from_slice(description_hash);
        data
    }

    /// Encode `delegate(address)` calldata.
    pub fn encode_delegate(delegatee: &str) -> Vec<u8> {
        let sel = Self::selector("delegate(address)");
        let mut data = sel.to_vec();
        // Address padded to 32 bytes (left-padded with zeros)
        let addr = delegatee.strip_prefix("0x").unwrap_or(delegatee);
        let addr_bytes = hex::decode(addr).unwrap_or_default();
        let mut padded = [0u8; 32];
        let start = 32 - addr_bytes.len().min(20);
        padded[start..start + addr_bytes.len().min(20)]
            .copy_from_slice(&addr_bytes[..addr_bytes.len().min(20)]);
        data.extend_from_slice(&padded);
        data
    }

    /// Encode `getVotes(address,uint256)` calldata.
    pub fn encode_get_votes(account: &str, block_number: u64) -> Vec<u8> {
        let sel = Self::selector("getVotes(address,uint256)");
        let mut data = sel.to_vec();
        // Address
        let addr = account.strip_prefix("0x").unwrap_or(account);
        let addr_bytes = hex::decode(addr).unwrap_or_default();
        let mut padded = [0u8; 32];
        let start = 32 - addr_bytes.len().min(20);
        padded[start..start + addr_bytes.len().min(20)]
            .copy_from_slice(&addr_bytes[..addr_bytes.len().min(20)]);
        data.extend_from_slice(&padded);
        // Block number
        let mut block_bytes = [0u8; 32];
        block_bytes[24..32].copy_from_slice(&block_number.to_be_bytes());
        data.extend_from_slice(&block_bytes);
        data
    }

    /// Return the hex-encoded selector for a function signature.
    pub fn selector_hex(signature: &str) -> String {
        format!("0x{}", hex::encode(Self::selector(signature)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_selector() {
        let sel = GovernorAbi::selector("propose(address[],uint256[],bytes[],string)");
        assert_eq!(sel.len(), 4);
        // Known selector for this signature
        assert_eq!(hex::encode(sel), "7d5e81e2");
    }

    #[test]
    fn cast_vote_selector() {
        let sel = GovernorAbi::selector("castVote(uint256,uint8)");
        assert_eq!(hex::encode(sel), "56781388");
    }

    #[test]
    fn delegate_selector() {
        let sel = GovernorAbi::selector("delegate(address)");
        assert_eq!(hex::encode(sel), "5c19a95c");
    }

    #[test]
    fn encode_cast_vote_length() {
        let data = GovernorAbi::encode_cast_vote(1, 1);
        assert_eq!(data.len(), 4 + 32 + 32); // selector + id + support
    }

    #[test]
    fn encode_delegate_contains_address() {
        let data = GovernorAbi::encode_delegate("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
        assert_eq!(data.len(), 4 + 32);
        // Last 20 bytes of the padded word should be the address
        let addr_part = &data[4 + 12..4 + 32];
        assert_eq!(
            hex::encode(addr_part),
            "d8da6bf26964af9d7eed9e03e53415d37aa96045"
        );
    }

    #[test]
    fn selector_hex_format() {
        let s = GovernorAbi::selector_hex("delegate(address)");
        assert!(s.starts_with("0x"));
        assert_eq!(s.len(), 10); // "0x" + 8 hex chars
    }
}
