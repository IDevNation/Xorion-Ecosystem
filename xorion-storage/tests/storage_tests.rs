use std::time::Duration;
use tempfile::TempDir;
use xorion_storage::{Encryption, FileCache, IpfsClient, PinningService, VirtualFs};

// ── Encryption integration tests ──────────────────────────

#[test]
fn encrypt_decrypt_roundtrip() {
    let enc = Encryption::from_password("xorion_secure", b"saltsaltsaltsalt").unwrap();
    let data = b"Xorion Web3 OS - decentralized storage layer";
    let encrypted = enc.encrypt(data).unwrap();
    let decrypted = enc.decrypt(&encrypted).unwrap();
    assert_eq!(&decrypted, data);
}

#[test]
fn wrong_password_cannot_decrypt() {
    let enc1 = Encryption::from_password("right", b"saltsaltsalt").unwrap();
    let enc2 = Encryption::from_password("wrong", b"saltsaltsalt").unwrap();
    let encrypted = enc1.encrypt(b"secret data").unwrap();
    assert!(enc2.decrypt(&encrypted).is_err());
}

#[test]
fn encryption_produces_different_ciphertexts() {
    let enc = Encryption::from_password("pass", b"saltsaltsalt").unwrap();
    let a = enc.encrypt(b"same").unwrap();
    let b = enc.encrypt(b"same").unwrap();
    assert_ne!(a, b); // random nonce
}

// ── VFS integration tests ─────────────────────────────────

#[test]
fn vfs_full_workflow() {
    let mut fs = VirtualFs::new();
    fs.mkdir("/photos").unwrap();
    fs.mkdir("/photos/vacation").unwrap();
    fs.add_file("/photos/vacation/beach.jpg", "QmBeach", 2048, false)
        .unwrap();
    fs.add_file("/photos/vacation/sunset.jpg", "QmSunset", 4096, true)
        .unwrap();

    let children = fs.ls("/photos/vacation").unwrap();
    assert_eq!(children.len(), 2);

    let entry = fs.get("/photos/vacation/sunset.jpg").unwrap();
    assert!(entry.encrypted);
    assert_eq!(entry.size, 4096);
}

#[test]
fn vfs_remove_then_readd() {
    let mut fs = VirtualFs::new();
    fs.add_file("/tmp.txt", "Qm1", 10, false).unwrap();
    fs.remove("/tmp.txt").unwrap();
    assert!(!fs.exists("/tmp.txt"));
    fs.add_file("/tmp.txt", "Qm2", 20, false).unwrap();
    assert_eq!(fs.get("/tmp.txt").unwrap().cid, "Qm2");
}

#[test]
fn vfs_json_export() {
    let mut fs = VirtualFs::new();
    fs.add_file("/data.bin", "QmData", 256, false).unwrap();
    let json = fs.to_json().unwrap();
    assert!(json.contains("QmData"));
    assert!(json.contains("data.bin"));
}

// ── Pinning integration tests ─────────────────────────────

#[test]
fn pinning_workflow() {
    let mut svc = PinningService::new();
    svc.pin("QmFile1", "my_document", 1024);
    svc.pin("QmFile2", "my_image", 2048);

    assert_eq!(svc.count(), 2);
    assert_eq!(svc.total_size(), 3072);
    assert!(svc.is_pinned("QmFile1"));

    svc.unpin("QmFile1").unwrap();
    assert!(!svc.is_pinned("QmFile1"));
    assert_eq!(svc.count(), 1);
}

// ── Cache integration tests ───────────────────────────────

#[test]
fn cache_put_get_remove() {
    let tmp = TempDir::new().unwrap();
    let mut cache = FileCache::new(tmp.path(), 1_000_000).unwrap();

    cache.put("QmA", b"alpha").unwrap();
    cache.put("QmB", b"bravo").unwrap();
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.get("QmA").unwrap(), b"alpha");

    cache.remove("QmA").unwrap();
    assert!(cache.get("QmA").is_none());
    assert_eq!(cache.len(), 1);
}

#[test]
fn cache_eviction_under_pressure() {
    let tmp = TempDir::new().unwrap();
    let mut cache = FileCache::new(tmp.path(), 100).unwrap();

    cache.put("Qm1", &[1u8; 60]).unwrap();
    cache.put("Qm2", &[2u8; 60]).unwrap();
    // Qm1 should have been evicted to make room for Qm2
    assert!(!cache.contains("Qm1"));
    assert!(cache.contains("Qm2"));
}

// ── IPFS client construction tests ────────────────────────

#[test]
fn ipfs_client_defaults() {
    let client = IpfsClient::new();
    assert_eq!(client.api_url(), "http://127.0.0.1:5001");
}

#[test]
fn ipfs_client_custom_config() {
    let client = IpfsClient::with_api_url("http://mynode:5001")
        .with_gateway("https://cloudflare-ipfs.com/ipfs")
        .with_timeout(Duration::from_secs(120));
    assert_eq!(client.api_url(), "http://mynode:5001");
    assert!(client.gateway_url().contains("cloudflare"));
}
