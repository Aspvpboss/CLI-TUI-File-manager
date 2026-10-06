use clipboard_rs::{Clipboard, ClipboardContext};
use fm_core::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tempfile::tempdir;

static CLIPBOARD_LOCK: Mutex<()> = Mutex::new(());

struct TestGuard(#[allow(dead_code)] std::sync::MutexGuard<'static, ()>);

fn acquire_lock() -> TestGuard {
    let guard = CLIPBOARD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _ = fs::remove_dir_all("file:");
    TestGuard(guard)
}

impl Drop for TestGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all("file:");
    }
}

fn get_clipboard_files() -> Vec<String> {
    let ctx = ClipboardContext::new().expect("failed to open clipboard context");
    ctx.get_files().unwrap_or_default()
}

// paste_files

#[test]
fn paste_files_without_delete_creates_file() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let target = dir.path().join("paste_no_delete.txt");

    copy_files(vec![&target]).unwrap();
    let cb_files = get_clipboard_files();
    assert_eq!(cb_files.len(), 1);
    let p = Path::new(&cb_files[0]);
    assert!(!p.exists());

    let res = paste_files(PasteDeleteRef::No);
    assert!(res.is_ok());
    assert!(p.is_file());
}

#[test]
fn paste_files_with_delete_creates_and_removes_file() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let target = dir.path().join("paste_with_delete.txt");

    copy_files(vec![&target]).unwrap();
    let cb_files = get_clipboard_files();
    assert_eq!(cb_files.len(), 1);
    let p = Path::new(&cb_files[0]);
    assert!(!p.exists());

    let res = paste_files(PasteDeleteRef::Yes);
    assert!(res.is_ok());
    assert!(!p.exists());
}

#[test]
fn paste_files_multiple_files_without_delete() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let t1 = dir.path().join("multi1.txt");
    let t2 = dir.path().join("multi2.txt");
    let t3 = dir.path().join("multi3.txt");

    copy_files(vec![&t1, &t2, &t3]).unwrap();
    let cb_files = get_clipboard_files();
    assert_eq!(cb_files.len(), 3);

    let res = paste_files(PasteDeleteRef::No);
    assert!(res.is_ok());

    for f in &cb_files {
        assert!(Path::new(f).is_file());
    }
}

#[test]
fn paste_files_multiple_files_with_delete() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let t1 = dir.path().join("multi_del1.txt");
    let t2 = dir.path().join("multi_del2.txt");
    let t3 = dir.path().join("multi_del3.txt");

    copy_files(vec![&t1, &t2, &t3]).unwrap();
    let cb_files = get_clipboard_files();
    assert_eq!(cb_files.len(), 3);

    let res = paste_files(PasteDeleteRef::Yes);
    assert!(res.is_ok());

    for f in &cb_files {
        assert!(!Path::new(f).exists());
    }
}

#[test]
fn paste_files_refuses_to_overwrite_existing_file() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let target = dir.path().join("already_exists.txt");

    copy_files(vec![&target]).unwrap();
    let cb_files = get_clipboard_files();
    let p = Path::new(&cb_files[0]);

    // First paste succeeds and creates the file
    paste_files(PasteDeleteRef::No).unwrap();
    assert!(p.is_file());

    // Second paste fails with AlreadyExists
    let res = paste_files(PasteDeleteRef::No);
    assert!(matches!(res, Err(FmError::AlreadyExists(_))));
}

#[test]
fn paste_files_already_exists_stops_before_deletion() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let target = dir.path().join("conflict_with_delete.txt");

    copy_files(vec![&target]).unwrap();
    let cb_files = get_clipboard_files();
    let p = Path::new(&cb_files[0]);

    // Create file initially via paste
    paste_files(PasteDeleteRef::No).unwrap();
    assert!(p.is_file());

    // Pasting with PasteDeleteRef::Yes encounters AlreadyExists during create_file
    let res = paste_files(PasteDeleteRef::Yes);
    assert!(matches!(res, Err(FmError::AlreadyExists(_))));

    // The file should still exist because deletion wasn't reached
    assert!(p.is_file());
}

#[test]
fn paste_files_empty_clipboard_succeeds() {
    let _guard = acquire_lock();
    let empty: Vec<PathBuf> = Vec::new();
    copy_files(empty).unwrap();

    let res_no = paste_files(PasteDeleteRef::No);
    assert!(res_no.is_ok());

    let res_yes = paste_files(PasteDeleteRef::Yes);
    assert!(res_yes.is_ok());
}

#[test]
fn paste_files_creates_missing_parent_directories() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let nested = dir.path().join("deep").join("nested").join("file.txt");

    copy_files(vec![&nested]).unwrap();
    let cb_files = get_clipboard_files();
    assert_eq!(cb_files.len(), 1);
    let p = Path::new(&cb_files[0]);

    let res = paste_files(PasteDeleteRef::No);
    assert!(res.is_ok());
    assert!(p.is_file());
    assert!(p.parent().unwrap().is_dir());
}
