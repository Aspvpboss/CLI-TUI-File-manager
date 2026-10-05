mod common;

use fm_core::{new, newf, rename};
use std::fs;

// Note: `rename` currently treats its second argument as a full destination path
// (it is passed straight to fs::rename), not just a new file name.

#[test]
fn rename_file_keeps_contents() {
    let dir = common::scratch("rename_file_keeps_contents");

    let old = dir.join("old.txt");
    let renamed = dir.join("new.txt");
    new(old.to_str().unwrap()).unwrap();
    fs::write(&old, "hello").unwrap();

    rename(old.to_str().unwrap(), renamed.to_str().unwrap()).unwrap();
    assert!(!old.exists());
    assert_eq!(fs::read_to_string(&renamed).unwrap(), "hello");

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn rename_folder_keeps_children() {
    let dir = common::scratch("rename_folder_keeps_children");

    let old = dir.join("old");
    let renamed = dir.join("new");
    new(old.join("child.txt").to_str().unwrap()).unwrap();

    rename(old.to_str().unwrap(), renamed.to_str().unwrap()).unwrap();
    assert!(!old.exists());
    assert!(renamed.join("child.txt").is_file());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn rename_missing_source_errors() {
    let dir = common::scratch("rename_missing_source_errors");

    let old = dir.join("missing.txt");
    let renamed = dir.join("new.txt");
    assert!(rename(old.to_str().unwrap(), renamed.to_str().unwrap()).is_err());
    assert!(!renamed.exists());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn rename_into_other_folder_moves() {
    let dir = common::scratch("rename_into_other_folder_moves");

    let file = dir.join("a.txt");
    let target_dir = dir.join("sub");
    let moved = target_dir.join("a.txt");
    new(file.to_str().unwrap()).unwrap();
    newf(target_dir.to_str().unwrap()).unwrap();

    rename(file.to_str().unwrap(), moved.to_str().unwrap()).unwrap();
    assert!(!file.exists());
    assert!(moved.is_file());

    fs::remove_dir_all(&dir).unwrap();
}
