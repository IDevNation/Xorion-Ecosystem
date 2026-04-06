//! Virtual filesystem backed by IPFS.
//!
//! Maps a directory tree (paths) to IPFS CIDs. Supports:
//! - Creating files and directories
//! - Listing directory contents
//! - Removing entries
//! - Optional client-side encryption per file

use std::collections::HashMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{Result, StorageError};

/// Metadata for a single entry in the virtual filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VfsEntry {
    /// IPFS CID of the content (empty for directories).
    pub cid: String,
    /// Entry name (last path component).
    pub name: String,
    /// File size in bytes (0 for directories).
    pub size: u64,
    /// Whether the content is client-side encrypted.
    pub encrypted: bool,
    /// Whether this is a directory.
    pub is_dir: bool,
    /// Creation timestamp (seconds since epoch).
    pub created: u64,
}

/// In-memory virtual filesystem that maps paths to IPFS CIDs.
pub struct VirtualFs {
    entries: HashMap<String, VfsEntry>,
}

impl VirtualFs {
    /// Create an empty virtual filesystem with a root directory.
    pub fn new() -> Self {
        let mut entries = HashMap::new();
        entries.insert(
            "/".to_string(),
            VfsEntry {
                cid: String::new(),
                name: "/".to_string(),
                size: 0,
                encrypted: false,
                is_dir: true,
                created: now_secs(),
            },
        );
        Self { entries }
    }

    /// Add a file entry at the given path.
    pub fn add_file(
        &mut self,
        path: &str,
        cid: &str,
        size: u64,
        encrypted: bool,
    ) -> Result<()> {
        let path = normalize_path(path);
        let parent = parent_path(&path);

        if !self.entries.contains_key(&parent) {
            return Err(StorageError::Filesystem(format!(
                "parent directory not found: {parent}"
            )));
        }

        let name = path.rsplit('/').next().unwrap_or(&path).to_string();

        self.entries.insert(
            path,
            VfsEntry {
                cid: cid.to_string(),
                name,
                size,
                encrypted,
                is_dir: false,
                created: now_secs(),
            },
        );

        Ok(())
    }

    /// Create a directory at the given path.
    pub fn mkdir(&mut self, path: &str) -> Result<()> {
        let path = normalize_path(path);
        let parent = parent_path(&path);

        if !self.entries.contains_key(&parent) {
            return Err(StorageError::Filesystem(format!(
                "parent directory not found: {parent}"
            )));
        }

        let name = path.rsplit('/').next().unwrap_or(&path).to_string();

        self.entries.insert(
            path,
            VfsEntry {
                cid: String::new(),
                name,
                size: 0,
                encrypted: false,
                is_dir: true,
                created: now_secs(),
            },
        );

        Ok(())
    }

    /// Get the entry at the given path.
    pub fn get(&self, path: &str) -> Result<&VfsEntry> {
        let path = normalize_path(path);
        self.entries
            .get(&path)
            .ok_or(StorageError::NotFound(path))
    }

    /// List immediate children of a directory.
    pub fn ls(&self, dir_path: &str) -> Result<Vec<&VfsEntry>> {
        let dir_path = normalize_path(dir_path);

        let entry = self
            .entries
            .get(&dir_path)
            .ok_or_else(|| StorageError::NotFound(dir_path.clone()))?;

        if !entry.is_dir {
            return Err(StorageError::Filesystem(format!(
                "not a directory: {dir_path}"
            )));
        }

        let prefix = if dir_path == "/" {
            "/".to_string()
        } else {
            format!("{dir_path}/")
        };

        let children: Vec<&VfsEntry> = self
            .entries
            .iter()
            .filter(|(k, _)| {
                if let Some(rest) = k.strip_prefix(&prefix) {
                    !rest.is_empty() && !rest.contains('/')
                } else {
                    false
                }
            })
            .map(|(_, v)| v)
            .collect();

        Ok(children)
    }

    /// Remove an entry (file or empty directory).
    pub fn remove(&mut self, path: &str) -> Result<()> {
        let path = normalize_path(path);

        if path == "/" {
            return Err(StorageError::Filesystem(
                "cannot remove root directory".into(),
            ));
        }

        let entry = self
            .entries
            .get(&path)
            .ok_or_else(|| StorageError::NotFound(path.clone()))?;

        if entry.is_dir && !self.ls(&path)?.is_empty() {
            return Err(StorageError::Filesystem(format!(
                "directory not empty: {path}"
            )));
        }

        self.entries.remove(&path);
        Ok(())
    }

    /// Check if a path exists.
    pub fn exists(&self, path: &str) -> bool {
        self.entries.contains_key(&normalize_path(path))
    }

    /// Total number of entries (including root).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the filesystem is empty (only root).
    pub fn is_empty(&self) -> bool {
        self.entries.len() <= 1
    }

    /// Export the entire filesystem tree as JSON.
    pub fn to_json(&self) -> Result<String> {
        let entries: Vec<&VfsEntry> = self.entries.values().collect();
        serde_json::to_string_pretty(&entries)
            .map_err(|e| StorageError::Filesystem(format!("serialization failed: {e}")))
    }
}

impl Default for VirtualFs {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalize a path: ensure leading `/`, remove trailing `/`.
fn normalize_path(path: &str) -> String {
    let path = path.trim();
    if path.is_empty() || path == "/" {
        return "/".to_string();
    }
    let mut p = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };
    if p.ends_with('/') && p.len() > 1 {
        p.pop();
    }
    p
}

/// Get the parent path of a normalized path.
fn parent_path(path: &str) -> String {
    if path == "/" {
        return "/".to_string();
    }
    match path.rfind('/') {
        Some(0) => "/".to_string(),
        Some(i) => path[..i].to_string(),
        None => "/".to_string(),
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
    fn root_exists_by_default() {
        let fs = VirtualFs::new();
        assert!(fs.exists("/"));
        assert!(fs.get("/").unwrap().is_dir);
    }

    #[test]
    fn add_file_to_root() {
        let mut fs = VirtualFs::new();
        fs.add_file("/readme.txt", "QmTest123", 42, false).unwrap();
        let entry = fs.get("/readme.txt").unwrap();
        assert_eq!(entry.cid, "QmTest123");
        assert_eq!(entry.size, 42);
        assert!(!entry.is_dir);
        assert!(!entry.encrypted);
    }

    #[test]
    fn add_encrypted_file() {
        let mut fs = VirtualFs::new();
        fs.add_file("/secret.dat", "QmEnc456", 100, true).unwrap();
        assert!(fs.get("/secret.dat").unwrap().encrypted);
    }

    #[test]
    fn mkdir_and_nested_file() {
        let mut fs = VirtualFs::new();
        fs.mkdir("/docs").unwrap();
        fs.add_file("/docs/paper.pdf", "QmPaper", 1024, false)
            .unwrap();
        let entry = fs.get("/docs/paper.pdf").unwrap();
        assert_eq!(entry.name, "paper.pdf");
    }

    #[test]
    fn mkdir_missing_parent_fails() {
        let mut fs = VirtualFs::new();
        let result = fs.mkdir("/a/b/c");
        assert!(result.is_err());
    }

    #[test]
    fn ls_root() {
        let mut fs = VirtualFs::new();
        fs.add_file("/a.txt", "Qm1", 1, false).unwrap();
        fs.add_file("/b.txt", "Qm2", 2, false).unwrap();
        fs.mkdir("/subdir").unwrap();
        let children = fs.ls("/").unwrap();
        assert_eq!(children.len(), 3);
    }

    #[test]
    fn ls_excludes_nested() {
        let mut fs = VirtualFs::new();
        fs.mkdir("/dir").unwrap();
        fs.add_file("/dir/file.txt", "Qm1", 10, false).unwrap();
        // /dir/file.txt should NOT appear in ls("/")
        let root_children = fs.ls("/").unwrap();
        assert_eq!(root_children.len(), 1);
        assert!(root_children[0].is_dir);
    }

    #[test]
    fn remove_file() {
        let mut fs = VirtualFs::new();
        fs.add_file("/tmp.txt", "Qm1", 1, false).unwrap();
        assert!(fs.exists("/tmp.txt"));
        fs.remove("/tmp.txt").unwrap();
        assert!(!fs.exists("/tmp.txt"));
    }

    #[test]
    fn remove_root_fails() {
        let mut fs = VirtualFs::new();
        assert!(fs.remove("/").is_err());
    }

    #[test]
    fn remove_nonempty_dir_fails() {
        let mut fs = VirtualFs::new();
        fs.mkdir("/dir").unwrap();
        fs.add_file("/dir/file", "Qm1", 1, false).unwrap();
        assert!(fs.remove("/dir").is_err());
    }

    #[test]
    fn normalize_paths() {
        assert_eq!(normalize_path(""), "/");
        assert_eq!(normalize_path("/"), "/");
        assert_eq!(normalize_path("foo"), "/foo");
        assert_eq!(normalize_path("/foo/"), "/foo");
        assert_eq!(normalize_path("/a/b"), "/a/b");
    }

    #[test]
    fn parent_paths() {
        assert_eq!(parent_path("/"), "/");
        assert_eq!(parent_path("/foo"), "/");
        assert_eq!(parent_path("/a/b"), "/a");
        assert_eq!(parent_path("/a/b/c"), "/a/b");
    }

    #[test]
    fn to_json_works() {
        let mut fs = VirtualFs::new();
        fs.add_file("/test.txt", "Qm123", 5, false).unwrap();
        let json = fs.to_json().unwrap();
        assert!(json.contains("Qm123"));
    }

    #[test]
    fn len_and_is_empty() {
        let mut fs = VirtualFs::new();
        assert!(fs.is_empty()); // only root
        fs.add_file("/a", "Qm1", 1, false).unwrap();
        assert!(!fs.is_empty());
        assert_eq!(fs.len(), 2); // root + file
    }
}
