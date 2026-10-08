
mod common;

use common::io_kind;
use fm_core::*;
use std::io::ErrorKind;
use tempfile::tempdir;

// remove_file

#[test]
fn remove_file_deletes_only_that_file() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    create_file(&a).unwrap();
    create_file(&b).unwrap();

    remove_file(&a).unwrap();

    assert!(!a.exists());
    assert!(b.is_file());
}

#[test]
fn remove_file_missing_is_not_found() {
    let dir = tempdir().unwrap();

    let err = remove_file(dir.path().join("nope.txt")).unwrap_err();

    assert_eq!(io_kind(err), ErrorKind::NotFound);
}

#[test]
fn remove_file_refuses_a_directory() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_dir(&sub).unwrap();

    assert!(matches!(remove_file(&sub), Err(FmError::Io(_))));
    assert!(sub.is_dir());
}
