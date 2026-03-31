//! Pin management for IPFS content.
//!
//! Tracks which CIDs are pinned locally, with metadata about
//! when they were pinned and an optional label.

use std::collections::HashMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{Result, StorageError};

/// Metadata about a pinned CID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinEntry {
    pub cid: String,
    pub label: String,
    pub pinned_at: u64,
    pub size: u64,
}

/// Local pin tracker. Manages the set of pinned CIDs with metadata.
pub struct PinningService {
    pins: HashMap<String, PinEntry>,
}

impl PinningService {
    /// Create an empty pinning service.
    pub fn new() -> Self {
        Self {
            pins: HashMap::new(),
        }
    }

    /// Mark a CID as pinned.
    pub fn pin(&mut self, cid: &str, label: &str, size: u64) {
        self.pins.insert(
            cid.to_string(),
            PinEntry {
                cid: cid.to_string(),
                label: label.to_string(),
                pinned_at: now_secs(),
                size,
            },
        );
    }

    /// Unpin a CID.
    pub fn unpin(&mut self, cid: &str) -> Result<()> {
        self.pins
            .remove(cid)
            .map(|_| ())
            .ok_or_else(|| StorageError::NotFound(format!("CID not pinned: {cid}")))
    }

    /// Check if a CID is pinned.
    pub fn is_pinned(&self, cid: &str) -> bool {
        self.pins.contains_key(cid)
    }

    /// Get metadata for a pinned CID.
    pub fn get(&self, cid: &str) -> Option<&PinEntry> {
        self.pins.get(cid)
    }

    /// List all pinned CIDs.
    pub fn list(&self) -> Vec<&PinEntry> {
        self.pins.values().collect()
    }

    /// Total number of pins.
    pub fn count(&self) -> usize {
        self.pins.len()
    }

    /// Total size of all pinned content.
    pub fn total_size(&self) -> u64 {
        self.pins.values().map(|p| p.size).sum()
    }

    /// Remove all pins.
    pub fn clear(&mut self) {
        self.pins.clear();
    }
}

impl Default for PinningService {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_and_check() {
        let mut svc = PinningService::new();
        svc.pin("QmTest1", "readme", 100);
        assert!(svc.is_pinned("QmTest1"));
        assert!(!svc.is_pinned("QmTest2"));
    }

    #[test]
    fn unpin() {
        let mut svc = PinningService::new();
        svc.pin("QmA", "a", 10);
        assert!(svc.is_pinned("QmA"));
        svc.unpin("QmA").unwrap();
        assert!(!svc.is_pinned("QmA"));
    }

    #[test]
    fn unpin_nonexistent_fails() {
        let mut svc = PinningService::new();
        assert!(svc.unpin("QmNope").is_err());
    }

    #[test]
    fn list_and_count() {
        let mut svc = PinningService::new();
        svc.pin("Qm1", "file1", 100);
        svc.pin("Qm2", "file2", 200);
        assert_eq!(svc.count(), 2);
        assert_eq!(svc.list().len(), 2);
    }

    #[test]
    fn total_size() {
        let mut svc = PinningService::new();
        svc.pin("Qm1", "a", 100);
        svc.pin("Qm2", "b", 250);
        assert_eq!(svc.total_size(), 350);
    }

    #[test]
    fn get_metadata() {
        let mut svc = PinningService::new();
        svc.pin("QmX", "my_file", 512);
        let entry = svc.get("QmX").unwrap();
        assert_eq!(entry.label, "my_file");
        assert_eq!(entry.size, 512);
        assert!(entry.pinned_at > 0);
    }

    #[test]
    fn clear_all() {
        let mut svc = PinningService::new();
        svc.pin("Qm1", "a", 10);
        svc.pin("Qm2", "b", 20);
        svc.clear();
        assert_eq!(svc.count(), 0);
    }
}
