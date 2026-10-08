
// Claude is responsible for most tests in this file

mod common;

use common::io_kind;
use fm_core::*;
use std::fs;
use std::io::ErrorKind;
use tempfile::tempdir;

// list_dir

fn names(entries: &[Entry]) -> Vec<&str> {
    entries.iter().map(|e| e.name.as_str()).collect()
}

#[test]
fn list_dir_of_empty_directory_is_empty() {
    let dir = tempdir().unwrap();

    assert!(list_dir(dir.path()).unwrap().is_empty());
}

#[test]
fn list_dir_reports_name_kind_and_size() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "hello").unwrap();
    fs::create_dir(dir.path().join("sub")).unwrap();

    let entries = list_dir(dir.path()).unwrap();

    assert_eq!(entries.len(), 2);
    let file = entries.iter().find(|e| e.name == "a.txt").unwrap();
    assert!(!file.is_dir);
    assert_eq!(file.size, 5);
    let sub = entries.iter().find(|e| e.name == "sub").unwrap();
    assert!(sub.is_dir);
}

#[test]
fn list_dir_sorts_dirs_first_then_by_name_ignoring_case() {
    let dir = tempdir().unwrap();
    create_file(dir.path().join("b.txt")).unwrap();
    create_file(dir.path().join("A.txt")).unwrap();
    create_file(dir.path().join("c.txt")).unwrap();
    create_dir(dir.path().join("Zeta")).unwrap();
    create_dir(dir.path().join("alpha")).unwrap();

    let entries = list_dir(dir.path()).unwrap();

    assert_eq!(
        names(&entries),
        ["alpha", "Zeta", "A.txt", "b.txt", "c.txt"]
    );
}

#[test]
fn list_dir_is_not_recursive() {
    let dir = tempdir().unwrap();
    create_file(dir.path().join("sub").join("inner.txt")).unwrap();

    let entries = list_dir(dir.path()).unwrap();

    assert_eq!(names(&entries), ["sub"]);
}

#[test]
fn list_dir_includes_dotfiles() {
    let dir = tempdir().unwrap();
    create_file(dir.path().join(".hidden")).unwrap();
    create_file(dir.path().join("shown.txt")).unwrap();

    let entries = list_dir(dir.path()).unwrap();

    assert_eq!(names(&entries), [".hidden", "shown.txt"]);
}

#[test]
fn list_dir_missing_is_not_found() {
    let dir = tempdir().unwrap();

    let err = list_dir(dir.path().join("nope")).unwrap_err();

    assert_eq!(io_kind(err), ErrorKind::NotFound);
}

#[test]
fn list_dir_refuses_a_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    create_file(&p).unwrap();

    assert!(matches!(list_dir(&p), Err(FmError::Io(_))));
}
