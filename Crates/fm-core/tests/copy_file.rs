// I hand made these terrible tests :)

use clipboard_rs::{ClipboardContext, Clipboard};
use fm_core::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_copy_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("burger.txt");
    let dir = dir.path().join("a");
    create_dir(&dir).unwrap();

    create_file(&p).unwrap();
    fs::write(&p, "Burger").unwrap();
    copy_file(&p).unwrap();

    let ctx = ClipboardContext::new().unwrap();
    let files = ctx.get_files().unwrap();

    println!("{files:?}");

    fs::copy(p, dir.join("burger.txt")).unwrap();

    // assert!(copy_file(p).is_ok());

}

