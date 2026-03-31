//! Example: Xorion IPFS Storage demo.

use xorion_ipfs::{Encryption, FileCache, PinningService, VirtualFs};

fn main() {
    println!("=== Xorion IPFS Storage Demo ===\n");

    // ── Encryption ─────────────────────────────────────
    println!("[1] Client-Side Encryption (AES-256-GCM)");
    let enc = Encryption::from_password("my_secure_password", b"xorion_salt_1234").unwrap();
    let plaintext = b"Top secret wallet backup data";
    let encrypted = enc.encrypt(plaintext).unwrap();
    println!("    Plaintext:  {} bytes", plaintext.len());
    println!("    Encrypted:  {} bytes (nonce + ciphertext + tag)", encrypted.len());
    let decrypted = enc.decrypt(&encrypted).unwrap();
    assert_eq!(&decrypted, plaintext);
    println!("    Decrypted:  OK\n");

    // ── Virtual Filesystem ─────────────────────────────
    println!("[2] Virtual Filesystem");
    let mut vfs = VirtualFs::new();
    vfs.mkdir("/wallet").unwrap();
    vfs.mkdir("/dapps").unwrap();
    vfs.add_file("/wallet/backup.enc", "QmBackup123", 2048, true).unwrap();
    vfs.add_file("/dapps/game.wasm", "QmGame456", 65536, false).unwrap();
    vfs.add_file("/readme.txt", "QmReadme789", 128, false).unwrap();

    println!("    Root contents:");
    for entry in vfs.ls("/").unwrap() {
        let kind = if entry.is_dir { "DIR " } else { "FILE" };
        let enc_flag = if entry.encrypted { " [encrypted]" } else { "" };
        println!("      {kind} {}{enc_flag}", entry.name);
    }

    println!("    /wallet contents:");
    for entry in vfs.ls("/wallet").unwrap() {
        println!("      {} ({} bytes, CID: {})", entry.name, entry.size, entry.cid);
    }
    println!();

    // ── Pinning Service ────────────────────────────────
    println!("[3] Pin Management");
    let mut pins = PinningService::new();
    pins.pin("QmBackup123", "wallet_backup", 2048);
    pins.pin("QmGame456", "game_dapp", 65536);
    println!("    Pinned: {} items, {} bytes total", pins.count(), pins.total_size());
    for pin in pins.list() {
        println!("      {} — {} ({} bytes)", pin.cid, pin.label, pin.size);
    }
    println!();

    // ── File Cache ─────────────────────────────────────
    println!("[4] Local File Cache");
    let tmp = tempfile::TempDir::new().unwrap();
    let mut cache = FileCache::new(tmp.path(), 1_000_000).unwrap();
    cache.put("QmReadme789", b"# Xorion Web3 OS\nThe future is decentralized.").unwrap();
    println!("    Cached: {} items, {} bytes", cache.len(), cache.size());
    let hit = cache.get("QmReadme789").unwrap();
    println!("    Cache hit: {} bytes", hit.len());

    println!("\nDone. All storage components operational.");
}
