
use fm_core::*;
use std::fs;
use tempfile::tempdir;

// create_dir

#[test]
fn create_dir_makes_a_directory() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");

    create_dir(&sub).unwrap();

    assert!(sub.is_dir());
}

#[test]
fn create_dir_makes_nested_directories() {
    let dir = tempdir().unwrap();
    let deep = dir.path().join("one").join("two").join("three");

    create_dir(&deep).unwrap();

    assert!(deep.is_dir());
}

#[test]
fn create_dir_refuses_existing_directory() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    let inner = sub.join("f.txt");
    create_file(&inner).unwrap();

    match create_dir(&sub) {
        Err(FmError::AlreadyExists(path)) => assert_eq!(path, sub),
        other => panic!("expected AlreadyExists, got {other:?}"),
    }
    assert!(inner.is_file());
}

#[test]
fn create_dir_refuses_existing_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("taken");
    fs::write(&p, "hello").unwrap();

    assert!(matches!(create_dir(&p), Err(FmError::AlreadyExists(_))));
    assert!(p.is_file());
    assert_eq!(fs::read_to_string(&p).unwrap(), "hello");
}
