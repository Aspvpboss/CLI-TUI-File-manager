// Claude is responsible for most tests in this file

mod common;

use common::io_kind;
use fm_core::*;
use std::fs;
use std::io::ErrorKind;
use tempfile::tempdir;

// rename

#[test]
fn rename_stays_in_same_dir() {
    let dir = tempdir().unwrap();
    let old = dir.path().join("old.txt");
    create_file(&old).unwrap();

    let new = rename(&old, "new.txt").unwrap();

    assert_eq!(new, dir.path().join("new.txt"));
    assert!(!old.exists());
    assert!(matches!(
        rename(&new, "../escape.txt"),
        Err(FmError::InvalidName(_))
    ));
}

#[test]
fn rename_keeps_file_contents() {
    let dir = tempdir().unwrap();
    let old = dir.path().join("old.txt");
    fs::write(&old, "hello").unwrap();

    let new = rename(&old, "new.txt").unwrap();

    assert_eq!(fs::read_to_string(&new).unwrap(), "hello");
}

#[test]
fn rename_directory_keeps_its_children() {
    let dir = tempdir().unwrap();
    let old = dir.path().join("old");
    fs::create_dir(&old).unwrap();
    fs::write(old.join("f.txt"), "hello").unwrap();

    let new = rename(&old, "new").unwrap();

    assert_eq!(new, dir.path().join("new"));
    assert!(!old.exists());
    assert_eq!(fs::read_to_string(new.join("f.txt")).unwrap(), "hello");
}

#[test]
fn rename_rejects_names_that_are_not_plain_names() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    create_file(&p).unwrap();

    for bad in ["", ".", "..", "sub/x.txt", "../x.txt", "/abs.txt", "x/"] {
        match rename(&p, bad) {
            Err(FmError::InvalidName(name)) => assert_eq!(name, bad),
            other => panic!("expected InvalidName for {bad:?}, got {other:?}"),
        }
    }
    assert!(p.is_file());
}

#[test]
fn rename_refuses_to_overwrite_a_file() {
    let dir = tempdir().unwrap();
    let one = dir.path().join("one.txt");
    let two = dir.path().join("two.txt");
    fs::write(&one, "one").unwrap();
    fs::write(&two, "two").unwrap();

    match rename(&one, "two.txt") {
        Err(FmError::AlreadyExists(path)) => assert_eq!(path, two),
        other => panic!("expected AlreadyExists, got {other:?}"),
    }
    assert_eq!(fs::read_to_string(&one).unwrap(), "one");
    assert_eq!(fs::read_to_string(&two).unwrap(), "two");
}

#[test]
fn rename_refuses_to_overwrite_a_directory() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let sub = dir.path().join("sub");
    create_file(&p).unwrap();
    create_dir(&sub).unwrap();

    assert!(matches!(rename(&p, "sub"), Err(FmError::AlreadyExists(_))));
    assert!(p.is_file());
    assert!(sub.is_dir());
}

#[test]
fn rename_missing_source_is_not_found() {
    let dir = tempdir().unwrap();

    let err = rename(dir.path().join("nope.txt"), "new.txt").unwrap_err();

    assert_eq!(io_kind(err), ErrorKind::NotFound);
    assert!(!dir.path().join("new.txt").exists());
}
