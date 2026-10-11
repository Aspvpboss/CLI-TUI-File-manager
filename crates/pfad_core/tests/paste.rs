use pfad_core::*;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use tempfile::tempdir;

// The clipboard is process-global, so tests that copy then paste must not interleave.
static SERIAL: Mutex<()> = Mutex::new(());

fn serial() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn paste_single_file() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let file = src.path().join("a.txt");
    fs::write(&file, "one").unwrap();

    copy_files(vec![&file]).unwrap();
    let created = paste_files(dest.path()).unwrap();

    assert_eq!(created, vec![std::path::absolute(dest.path()).unwrap().join("a.txt")]);
    assert_eq!(fs::read_to_string(dest.path().join("a.txt")).unwrap(), "one");
    // The source is copied, not moved.
    assert!(file.exists());
}

#[test]
fn paste_directory_recursively() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let sub = src.path().join("sub");
    fs::create_dir_all(sub.join("nested").join("deep")).unwrap();
    fs::write(sub.join("top.txt"), "top").unwrap();
    fs::write(sub.join("nested").join("mid.txt"), "mid").unwrap();
    fs::write(sub.join("nested").join("deep").join("low.txt"), "low").unwrap();

    copy_files(vec![&sub]).unwrap();
    paste_files(dest.path()).unwrap();

    let out = dest.path().join("sub");
    assert_eq!(fs::read_to_string(out.join("top.txt")).unwrap(), "top");
    assert_eq!(fs::read_to_string(out.join("nested").join("mid.txt")).unwrap(), "mid");
    assert_eq!(
        fs::read_to_string(out.join("nested").join("deep").join("low.txt")).unwrap(),
        "low"
    );
    assert!(sub.join("top.txt").exists());
}

#[test]
fn paste_empty_directory() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let empty = src.path().join("empty");
    fs::create_dir(&empty).unwrap();

    copy_files(vec![&empty]).unwrap();
    paste_files(dest.path()).unwrap();

    assert!(dest.path().join("empty").is_dir());
}

#[test]
fn paste_mixed_files_and_dirs() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let a = src.path().join("a.txt");
    let b = src.path().join("b.txt");
    let sub = src.path().join("sub");
    fs::write(&a, "a").unwrap();
    fs::write(&b, "b").unwrap();
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("inner.txt"), "inner").unwrap();

    copy_files(vec![a, b, sub]).unwrap();
    let created = paste_files(dest.path()).unwrap();

    assert_eq!(created.len(), 3);
    assert_eq!(fs::read_to_string(dest.path().join("a.txt")).unwrap(), "a");
    assert_eq!(fs::read_to_string(dest.path().join("b.txt")).unwrap(), "b");
    assert_eq!(
        fs::read_to_string(dest.path().join("sub").join("inner.txt")).unwrap(),
        "inner"
    );
}

#[test]
fn paste_existing_file_is_an_error_and_not_overwritten() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let file = src.path().join("a.txt");
    fs::write(&file, "new").unwrap();
    fs::write(dest.path().join("a.txt"), "old").unwrap();

    copy_files(vec![&file]).unwrap();
    let err = paste_files(dest.path()).unwrap_err();

    assert!(matches!(err, PfadError::AlreadyExists(_)));
    assert_eq!(fs::read_to_string(dest.path().join("a.txt")).unwrap(), "old");
}

#[test]
fn paste_stops_at_first_collision_but_keeps_earlier_copies() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let a = src.path().join("a.txt");
    let b = src.path().join("b.txt");
    fs::write(&a, "a").unwrap();
    fs::write(&b, "b").unwrap();
    fs::write(dest.path().join("b.txt"), "old").unwrap();

    copy_files(vec![&a, &b]).unwrap();
    let err = paste_files(dest.path()).unwrap_err();

    assert!(matches!(err, PfadError::AlreadyExists(_)));
    assert!(dest.path().join("a.txt").exists());
    assert_eq!(fs::read_to_string(dest.path().join("b.txt")).unwrap(), "old");
}

#[test]
fn paste_dir_into_its_own_subdir_is_an_error() {
    let _g = serial();
    let src = tempdir().unwrap();
    let sub = src.path().join("sub");
    let inner = sub.join("inner");
    fs::create_dir_all(&inner).unwrap();

    copy_files(vec![&sub]).unwrap();
    let err = paste_files(&inner).unwrap_err();

    assert!(matches!(err, PfadError::PasteIntoSelf(_)));
    assert!(!inner.join("sub").exists());
}

#[test]
fn paste_after_clear_does_not_reuse_old_selection() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let file = src.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    copy_files(vec![&file]).unwrap();
    copy_files(Vec::<PathBuf>::new()).unwrap();

    assert!(paste_files(dest.path()).unwrap().is_empty());
    assert_eq!(fs::read_dir(dest.path()).unwrap().count(), 0);
}

#[test]
fn paste_uses_latest_copy() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let a = src.path().join("a.txt");
    let b = src.path().join("b.txt");
    fs::write(&a, "a").unwrap();
    fs::write(&b, "b").unwrap();

    copy_files(vec![&a]).unwrap();
    copy_files(vec![&b]).unwrap();
    paste_files(dest.path()).unwrap();

    assert!(!dest.path().join("a.txt").exists());
    assert!(dest.path().join("b.txt").exists());
}

#[test]
fn paste_into_missing_destination_is_an_io_error() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let file = src.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    copy_files(vec![&file]).unwrap();
    let err = paste_files(dest.path().join("nope")).unwrap_err();

    assert!(matches!(err, PfadError::Io(_)));
}

#[test]
fn paste_path_with_spaces_and_unicode() {
    let _g = serial();
    let src = tempdir().unwrap();
    let dest = tempdir().unwrap();
    let name = "my file – ünïcödé 文件.txt";
    let file = src.path().join(name);
    fs::write(&file, "x").unwrap();

    copy_files(vec![&file]).unwrap();
    paste_files(dest.path()).unwrap();

    assert!(dest.path().join(name).exists());
}
