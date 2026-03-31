//! Proof caching with TTL-based expiration.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// A cached proof entry with creation timestamp.
struct CachedEntry {
    proof_bytes: Vec<u8>,
    created: Instant,
}

/// Thread-safe cache for generated proofs.
///
/// Avoids redundant re-computation of expensive Groth16 proofs
/// by caching them with a configurable time-to-live.
pub struct ProofCache {
    entries: Mutex<HashMap<String, CachedEntry>>,
    ttl: Duration,
}

impl ProofCache {
    /// Create a new cache with the given TTL in seconds.
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    /// Look up a cached proof by key. Returns `None` if missing or expired.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let entries = self.entries.lock().unwrap();
        entries.get(key).and_then(|entry| {
            if entry.created.elapsed() < self.ttl {
                Some(entry.proof_bytes.clone())
            } else {
                None
            }
        })
    }

    /// Insert a proof into the cache.
    pub fn insert(&self, key: String, proof_bytes: Vec<u8>) {
        let mut entries = self.entries.lock().unwrap();
        entries.insert(
            key,
            CachedEntry {
                proof_bytes,
                created: Instant::now(),
            },
        );
    }

    /// Remove all expired entries.
    pub fn evict_expired(&self) -> usize {
        let mut entries = self.entries.lock().unwrap();
        let before = entries.len();
        entries.retain(|_, entry| entry.created.elapsed() < self.ttl);
        before - entries.len()
    }

    /// Return the number of cached entries (including expired).
    pub fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.lock().unwrap().is_empty()
    }

    /// Clear all entries.
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn insert_and_retrieve() {
        let cache = ProofCache::new(60);
        cache.insert("key1".into(), vec![1, 2, 3]);
        assert_eq!(cache.get("key1"), Some(vec![1, 2, 3]));
    }

    #[test]
    fn missing_key_returns_none() {
        let cache = ProofCache::new(60);
        assert_eq!(cache.get("nonexistent"), None);
    }

    #[test]
    fn expired_entry_returns_none() {
        let cache = ProofCache::new(0); // 0-second TTL
        cache.insert("key".into(), vec![1, 2, 3]);
        thread::sleep(Duration::from_millis(10));
        assert_eq!(cache.get("key"), None);
    }

    #[test]
    fn evict_expired_entries() {
        let cache = ProofCache::new(0);
        cache.insert("a".into(), vec![1]);
        cache.insert("b".into(), vec![2]);
        thread::sleep(Duration::from_millis(10));
        let evicted = cache.evict_expired();
        assert_eq!(evicted, 2);
        assert!(cache.is_empty());
    }

    #[test]
    fn len_and_clear() {
        let cache = ProofCache::new(60);
        assert!(cache.is_empty());
        cache.insert("a".into(), vec![1]);
        cache.insert("b".into(), vec![2]);
        assert_eq!(cache.len(), 2);
        cache.clear();
        assert!(cache.is_empty());
    }
}
