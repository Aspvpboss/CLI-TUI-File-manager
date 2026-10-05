mod common;

use fm_core::{del, delf, new, newf};
use std::fs;

#[test]
fn delete_file() {
    let dir = common::scratch("delete_file");

    let file = dir.join("gone.txt");
    new(file.to_str().unwrap()).unwrap();
    del(file.to_str().unwrap()).unwrap();
    assert!(!file.exists());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn delete_missing_file_errors() {
    let dir = common::scratch("delete_missing_file_errors");

    let file = dir.join("nope.txt");
    assert!(del(file.to_str().unwrap()).is_err());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn del_does_not_remove_folders() {
    let dir = common::scratch("del_does_not_remove_folders");

    let folder = dir.join("keep");
    newf(folder.to_str().unwrap()).unwrap();
    assert!(del(folder.to_str().unwrap()).is_err());
    assert!(folder.is_dir());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn delete_folder_recursively() {
    let dir = common::scratch("delete_folder_recursively");

    let folder = dir.join("top");
    new(folder.join("inner/file.txt").to_str().unwrap()).unwrap();
    delf(folder.to_str().unwrap()).unwrap();
    assert!(!folder.exists());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn delete_missing_folder_errors() {
    let dir = common::scratch("delete_missing_folder_errors");

    let folder = dir.join("nope");
    assert!(delf(folder.to_str().unwrap()).is_err());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn delf_does_not_remove_files() {
    let dir = common::scratch("delf_does_not_remove_files");

    let file = dir.join("keep.txt");
    new(file.to_str().unwrap()).unwrap();
    assert!(delf(file.to_str().unwrap()).is_err());
    assert!(file.is_file());

    fs::remove_dir_all(&dir).unwrap();
}
