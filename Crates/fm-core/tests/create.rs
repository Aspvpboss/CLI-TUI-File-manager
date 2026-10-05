mod common;

use fm_core::{new, newf};
use std::fs;

#[test]
fn create_file_and_parents() {
    let dir = common::scratch("create_file_and_parents");

    let file = dir.join("a/b/c.txt");
    new(file.to_str().unwrap()).unwrap();
    assert!(file.is_file());

    let folder = dir.join("x/y");
    newf(folder.to_str().unwrap()).unwrap();
    assert!(folder.is_dir());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn create_file_in_existing_dir() {
    let dir = common::scratch("create_file_in_existing_dir");

    let file = dir.join("plain.txt");
    new(file.to_str().unwrap()).unwrap();
    assert!(file.is_file());
    assert_eq!(fs::metadata(&file).unwrap().len(), 0);

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn create_folder_is_idempotent() {
    let dir = common::scratch("create_folder_is_idempotent");

    let folder = dir.join("same");
    newf(folder.to_str().unwrap()).unwrap();
    newf(folder.to_str().unwrap()).unwrap();
    assert!(folder.is_dir());

    fs::remove_dir_all(&dir).unwrap();
}
