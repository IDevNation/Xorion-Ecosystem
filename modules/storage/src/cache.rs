//! Local file cache with size-based eviction.
//!
//! Stores fetched IPFS content on disk to avoid redundant downloads.
//! When the cache exceeds the configured size limit, the oldest
//! entries are evicted first.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{Result, StorageError};

/// Entry metadata tracked in memory.
#[derive(Debug, Clone)]
struct CacheEntry {
    size: u64,
    last_access: u64,
}

/// Disk-backed LRU cache for IPFS content.
pub struct FileCache {
    dir: PathBuf,
    max_size: u64,
    entries: BTreeMap<String, CacheEntry>,
    current_size: u64,
}

impl FileCache {
    /// Create a new cache at the given directory with a max size in bytes.
    pub fn new(dir: &Path, max_size: u64) -> Result<Self> {
        std::fs::create_dir_all(dir)
            .map_err(|e| StorageError::Cache(format!("create cache dir failed: {e}")))?;

        Ok(Self {
            dir: dir.to_path_buf(),
            max_size,
            entries: BTreeMap::new(),
            current_size: 0,
        })
    }

    /// Get cached content by CID. Returns `None` on miss.
    pub fn get(&mut self, cid: &str) -> Option<Vec<u8>> {
        if !self.entries.contains_key(cid) {
            return None;
        }

        let path = self.path_for(cid);
        match std::fs::read(&path) {
            Ok(data) => {
                // Update access time
                if let Some(entry) = self.entries.get_mut(cid) {
                    entry.last_access = now_secs();
                }
                Some(data)
            }
            Err(_) => {
                // File disappeared — remove from index
                self.entries.remove(cid);
                None
            }
        }
    }

    /// Insert content into the cache, evicting old entries if needed.
    pub fn put(&mut self, cid: &str, data: &[u8]) -> Result<()> {
        let size = data.len() as u64;

        // Don't cache if single item exceeds limit
        if size > self.max_size {
            return Ok(());
        }

        // Evict until there's room
        while self.current_size + size > self.max_size {
            if !self.evict_oldest() {
                break;
            }
        }

        let path = self.path_for(cid);
        std::fs::write(&path, data)
            .map_err(|e| StorageError::Cache(format!("write cache file failed: {e}")))?;

        self.current_size += size;
        self.entries.insert(
            cid.to_string(),
            CacheEntry {
                size,
                last_access: now_secs(),
            },
        );

        Ok(())
    }

    /// Remove a specific entry from the cache.
    pub fn remove(&mut self, cid: &str) -> Result<()> {
        if let Some(entry) = self.entries.remove(cid) {
            self.current_size = self.current_size.saturating_sub(entry.size);
            let path = self.path_for(cid);
            let _ = std::fs::remove_file(path);
        }
        Ok(())
    }

    /// Check if a CID is cached.
    pub fn contains(&self, cid: &str) -> bool {
        self.entries.contains_key(cid)
    }

    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current total size of cached data.
    pub fn size(&self) -> u64 {
        self.current_size
    }

    /// Clear all cached entries and files.
    pub fn clear(&mut self) -> Result<()> {
        for cid in self.entries.keys().cloned().collect::<Vec<_>>() {
            let path = self.path_for(&cid);
            let _ = std::fs::remove_file(path);
        }
        self.entries.clear();
        self.current_size = 0;
        Ok(())
    }

    /// File path for a given CID.
    fn path_for(&self, cid: &str) -> PathBuf {
        // Replace any path-unsafe characters
        let safe_name: String = cid.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
        self.dir.join(safe_name)
    }

    /// Evict the oldest entry. Returns false if nothing to evict.
    fn evict_oldest(&mut self) -> bool {
        let oldest_cid = self
            .entries
            .iter()
            .min_by_key(|(_, e)| e.last_access)
            .map(|(k, _)| k.clone());

        if let Some(cid) = oldest_cid {
            if let Some(entry) = self.entries.remove(&cid) {
                self.current_size = self.current_size.saturating_sub(entry.size);
                let path = self.path_for(&cid);
                let _ = std::fs::remove_file(path);
                return true;
            }
        }
        false
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_cache(max_size: u64) -> (FileCache, TempDir) {
        let tmp = TempDir::new().unwrap();
        let cache = FileCache::new(tmp.path(), max_size).unwrap();
        (cache, tmp)
    }

    #[test]
    fn put_and_get() {
        let (mut cache, _tmp) = test_cache(1_000_000);
        cache.put("QmTest1", b"hello world").unwrap();
        assert_eq!(cache.get("QmTest1").unwrap(), b"hello world");
    }

    #[test]
    fn miss_returns_none() {
        let (mut cache, _tmp) = test_cache(1_000_000);
        assert!(cache.get("QmNope").is_none());
    }

    #[test]
    fn contains_check() {
        let (mut cache, _tmp) = test_cache(1_000_000);
        assert!(!cache.contains("Qm1"));
        cache.put("Qm1", b"data").unwrap();
        assert!(cache.contains("Qm1"));
    }

    #[test]
    fn remove_entry() {
        let (mut cache, _tmp) = test_cache(1_000_000);
        cache.put("Qm1", b"data").unwrap();
        cache.remove("Qm1").unwrap();
        assert!(!cache.contains("Qm1"));
        assert!(cache.is_empty());
    }

    #[test]
    fn eviction_when_full() {
        // 50-byte limit: first entry is 30 bytes, second is 30 bytes
        // -> first should be evicted
        let (mut cache, _tmp) = test_cache(50);
        cache.put("Qm1", &[0u8; 30]).unwrap();
        cache.put("Qm2", &[1u8; 30]).unwrap();
        assert!(!cache.contains("Qm1")); // evicted
        assert!(cache.contains("Qm2"));
    }

    #[test]
    fn skip_oversized() {
        let (mut cache, _tmp) = test_cache(10);
        cache.put("QmBig", &[0u8; 100]).unwrap(); // silently skipped
        assert!(!cache.contains("QmBig"));
    }

    #[test]
    fn clear_all() {
        let (mut cache, _tmp) = test_cache(1_000_000);
        cache.put("Qm1", b"a").unwrap();
        cache.put("Qm2", b"b").unwrap();
        cache.clear().unwrap();
        assert!(cache.is_empty());
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn size_tracking() {
        let (mut cache, _tmp) = test_cache(1_000_000);
        cache.put("Qm1", &[0u8; 100]).unwrap();
        cache.put("Qm2", &[0u8; 200]).unwrap();
        assert_eq!(cache.size(), 300);
        cache.remove("Qm1").unwrap();
        assert_eq!(cache.size(), 200);
    }
}
