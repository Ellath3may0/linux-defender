//! Integration tests for `src/backups/`. One submodule per source module.
//!
//! Submodules carry explicit `#[path]` attributes because a test crate root
//! resolves `mod` declarations against `tests/`, not `tests/backups/`.

#[path = "common.rs"]
mod common;

#[path = "backups/all.rs"]
mod all;

#[path = "backups/compare_hashes.rs"]
mod compare_hashes;

#[path = "backups/hash.rs"]
mod hash;

use tempfile::TempDir;

use common::{run_cli, stderr, stdout};

/// Backup kinds that are not implemented yet; they must still route correctly.
const STUB_KINDS: &[(&str, &str)] = &[
    ("--custom", "custom service"),
    ("--restore", "Restoring from backup"),
];

#[test]
fn every_backup_kind_is_wired_to_its_module() {
    let dir = TempDir::new().unwrap();

    for (flag, banner) in STUB_KINDS {
        let out = run_cli(dir.path(), &["backups", flag]);
        assert!(out.status.success(), "{} should exit cleanly", flag);
        assert!(
            stdout(&out).contains(banner),
            "{} should announce {:?}, got: {}",
            flag,
            banner,
            stdout(&out)
        );
    }
}

#[test]
fn several_backup_kinds_can_run_in_one_invocation() {
    let dir = TempDir::new().unwrap();
    let out = run_cli(
        dir.path(),
        &[
            "backups",
            "--mail",
            "--splunk",
            "--save-location",
            "~/backup-concurrency-test-dir",
        ],
    );

    assert!(out.status.success());
    assert!(
        stdout(&out).contains("[+] Backed up /etc/postfix")
            || stderr(&out).contains("[X] Failed to back up /etc/postfix")
            || stderr(&out).contains("[!] /etc/postfix does not exist, skipping")
    );
    assert!(
        stdout(&out).contains("[+] Backed up /opt/splunk/")
            || stderr(&out).contains("[X] Failed to back up /opt/splunk/")
            || stderr(&out).contains("[!] /opt/splunk/ does not exist, skipping")
    );
}
