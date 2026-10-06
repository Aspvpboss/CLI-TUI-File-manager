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

fn assert_clipboard_matches_path(clipboard_entry: &str, expected_path: &Path) {
    let expected_str = expected_path.to_str().unwrap();
    let expected_uri = format!("file://{}", expected_path.display());
    assert!(
        clipboard_entry == expected_str || clipboard_entry == expected_uri,
        "clipboard entry {:?} did not match expected path {:?} (or uri {:?})",
        clipboard_entry,
        expected_str,
        expected_uri
    );
}

// copy_files

#[test]
fn copy_files_single_file_populates_clipboard() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    create_file(&file).unwrap();

    let res = copy_files(vec![&file]);
    assert!(res.is_ok());

    let files = get_clipboard_files();
    assert_eq!(files.len(), 1);
    assert_clipboard_matches_path(&files[0], &file);
}

#[test]
fn copy_files_multiple_files_populates_clipboard() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("one.txt");
    let file2 = dir.path().join("two.txt");
    let file3 = dir.path().join("three.txt");
    create_file(&file1).unwrap();
    create_file(&file2).unwrap();
    create_file(&file3).unwrap();

    let res = copy_files(vec![&file1, &file2, &file3]);
    assert!(res.is_ok());

    let files = get_clipboard_files();
    assert_eq!(files.len(), 3);
    assert_clipboard_matches_path(&files[0], &file1);
    assert_clipboard_matches_path(&files[1], &file2);
    assert_clipboard_matches_path(&files[2], &file3);
}

#[test]
fn copy_files_empty_list_clears_clipboard() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let file = dir.path().join("before.txt");
    create_file(&file).unwrap();
    copy_files(vec![&file]).unwrap();
    assert!(!get_clipboard_files().is_empty());

    let empty: Vec<PathBuf> = Vec::new();
    let res = copy_files(empty);
    assert!(res.is_ok());

    let files = get_clipboard_files();
    assert!(files.is_empty());
}

#[test]
fn copy_files_overwrites_previous_selection() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("first.txt");
    let file2 = dir.path().join("second.txt");
    create_file(&file1).unwrap();
    create_file(&file2).unwrap();

    copy_files(vec![&file1]).unwrap();
    let files = get_clipboard_files();
    assert_eq!(files.len(), 1);
    assert_clipboard_matches_path(&files[0], &file1);

    copy_files(vec![&file2]).unwrap();
    let files = get_clipboard_files();
    assert_eq!(files.len(), 1);
    assert_clipboard_matches_path(&files[0], &file2);
}

#[test]
fn copy_files_accepts_different_path_types() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let path = dir.path().join("typed.txt");
    create_file(&path).unwrap();

    // AsRef<Path> with &Path
    let res = copy_files(vec![path.as_path()]);
    assert!(res.is_ok());

    // AsRef<Path> with PathBuf
    let res = copy_files(vec![path.clone()]);
    assert!(res.is_ok());

    // AsRef<Path> with &str
    let path_str = path.to_str().unwrap();
    let res = copy_files(vec![path_str]);
    assert!(res.is_ok());

    // AsRef<Path> with String
    let res = copy_files(vec![path_str.to_string()]);
    assert!(res.is_ok());
}

#[test]
fn copy_files_supports_directories() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let sub = dir.path().join("folder");
    create_dir(&sub).unwrap();

    let res = copy_files(vec![&sub]);
    assert!(res.is_ok());

    let files = get_clipboard_files();
    assert_eq!(files.len(), 1);
    assert_clipboard_matches_path(&files[0], &sub);
}

#[test]
fn copy_files_supports_non_existent_paths() {
    let _guard = acquire_lock();
    let dir = tempdir().unwrap();
    let missing = dir.path().join("ghost.txt");
    assert!(!missing.exists());

    let res = copy_files(vec![&missing]);
    assert!(res.is_ok());

    let files = get_clipboard_files();
    assert_eq!(files.len(), 1);
    assert_clipboard_matches_path(&files[0], &missing);
}
