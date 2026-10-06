// I hand made these terrible tests :)

use fm_core::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_copy_file() {
    // let dir = tempdir().unwrap();
    // let p = dir.path().join("a.txt");

    let p = "burger.txt";

    create_file(&p).unwrap();

    assert!(copy_file(p).is_err());

}