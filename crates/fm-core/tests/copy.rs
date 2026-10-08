
// copy_files
//
// `copy_files` only records absolute paths (system clipboard, or an internal
// fallback list), so these tests can only check the return value and that the
// source files are left alone.

use fm_core::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use tempfile::tempdir;

#[test]
fn copy_single_file_succeeds() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "one").unwrap();

    assert!(copy_files(vec![&file]).is_ok());
}

#[test]
fn copy_multiple_files_succeeds() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    let c = dir.path().join("c.txt");
    for p in [&a, &b, &c] {
        fs::write(p, "x").unwrap();
    }

    assert!(copy_files(vec![a, b, c]).is_ok());
}

#[test]
fn copy_directory_succeeds() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir_all(sub.join("nested")).unwrap();
    fs::write(sub.join("nested").join("f.txt"), "x").unwrap();

    assert!(copy_files(vec![&sub]).is_ok());
}

#[test]
fn copy_mixed_files_and_dirs_succeeds() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let sub = dir.path().join("sub");
    fs::write(&file, "x").unwrap();
    fs::create_dir(&sub).unwrap();

    assert!(copy_files(vec![file, sub]).is_ok());
}

#[test]
fn copy_empty_list_succeeds() {
    assert!(copy_files(Vec::<PathBuf>::new()).is_ok());
}

#[test]
fn copy_accepts_relative_path() {
    assert!(copy_files(vec!["Cargo.toml"]).is_ok());
}

#[test]
fn copy_missing_path_is_accepted() {
    let dir = tempdir().unwrap();
    let missing = dir.path().join("does_not_exist");

    assert!(copy_files(vec![&missing]).is_ok());
    assert!(!missing.exists());
}

#[test]
fn copy_accepts_str_pathbuf_and_path_refs() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    let as_str = file.to_str().unwrap();
    assert!(copy_files(vec![as_str]).is_ok());
    assert!(copy_files(vec![file.clone()]).is_ok());
    assert!(copy_files(vec![file.as_path()]).is_ok());
    assert!(copy_files(Vec::<&Path>::new()).is_ok());
}

#[test]
fn copy_does_not_modify_source() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let sub = dir.path().join("sub");
    fs::write(&file, "one").unwrap();
    fs::create_dir(&sub).unwrap();

    copy_files(vec![&file, &sub]).unwrap();

    assert_eq!(fs::read_to_string(&file).unwrap(), "one");
    assert!(sub.is_dir());
}

#[test]
fn copy_twice_succeeds() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    fs::write(&a, "a").unwrap();
    fs::write(&b, "b").unwrap();

    assert!(copy_files(vec![&a]).is_ok());
    assert!(copy_files(vec![&b]).is_ok());
    assert!(copy_files(Vec::<PathBuf>::new()).is_ok());
}

#[cfg(unix)]
#[test]
fn copy_non_utf8_path_falls_back_and_succeeds() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let dir = tempdir().unwrap();
    let weird = dir.path().join(OsStr::from_bytes(b"bad\xff\xfename"));
    fs::write(&weird, "x").unwrap();

    assert!(copy_files(vec![&weird]).is_ok());
}

#[test]
fn copy_is_safe_across_threads() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let file = file.clone();
            thread::spawn(move || {
                for _ in 0..10 {
                    copy_files(vec![&file]).unwrap();
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
