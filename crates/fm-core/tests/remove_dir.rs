
// Claude is responsible for most tests in this file

mod common;

use common::io_kind;
use fm_core::*;
use std::fs;
use std::io::ErrorKind;
use tempfile::tempdir;

// remove_dir

#[test]
fn remove_dir_non_recursive_removes_empty_dir() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_dir(&sub).unwrap();

    remove_dir(&sub, Recursion::No).unwrap();

    assert!(!sub.exists());
}

#[test]
fn non_recursive_remove_fails_on_full_dir() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_file(sub.join("f.txt")).unwrap();

    assert!(remove_dir(&sub, Recursion::No).is_err());
    assert!(remove_dir(&sub, Recursion::Yes).is_ok());
}

#[test]
fn failed_non_recursive_remove_leaves_contents() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    let inner = sub.join("f.txt");
    fs::create_dir(&sub).unwrap();
    fs::write(&inner, "hello").unwrap();

    assert!(remove_dir(&sub, Recursion::No).is_err());

    assert!(sub.is_dir());
    assert_eq!(fs::read_to_string(&inner).unwrap(), "hello");
}

#[test]
fn remove_dir_recursive_removes_whole_tree() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    let sibling = dir.path().join("sibling.txt");
    create_file(sub.join("a").join("b").join("deep.txt")).unwrap();
    create_file(sub.join("top.txt")).unwrap();
    create_file(&sibling).unwrap();

    remove_dir(&sub, Recursion::Yes).unwrap();

    assert!(!sub.exists());
    assert!(sibling.is_file());
}

#[test]
fn remove_dir_missing_is_not_found() {
    let dir = tempdir().unwrap();
    let missing = dir.path().join("nope");

    for recursion in [Recursion::No, Recursion::Yes] {
        let err = remove_dir(&missing, recursion).unwrap_err();
        assert_eq!(io_kind(err), ErrorKind::NotFound);
    }
}

#[test]
fn remove_dir_refuses_a_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    fs::write(&p, "hello").unwrap();

    for recursion in [Recursion::No, Recursion::Yes] {
        assert!(matches!(remove_dir(&p, recursion), Err(FmError::Io(_))));
    }
    assert_eq!(fs::read_to_string(&p).unwrap(), "hello");
}
