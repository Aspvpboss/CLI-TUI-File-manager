// I hand made these terrible tests :)

use fm_core::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_copy_file() {
    // let dir = tempdir().unwrap();
    // let p = dir.path().join("a.txt");

    let p = "home/aspvpboss17306/Documents/programs/Workbench/CLI-file-explorer/Crates/fm-core/burger.txt";
    if let Ok(var) = fs::exists("burger.txt"){
        if var == false {
            create_file("burger.txt").unwrap();
        }
    }

    // create_file(&p).unwrap();

    assert!(copy_file(p).is_ok());

}