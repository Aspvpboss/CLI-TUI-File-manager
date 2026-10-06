// I hand made these terrible tests :)

use clipboard_rs::{ClipboardContext, Clipboard};
use fm_core::*;
use std::fs;
use tempfile::tempdir;
use std::thread::sleep;

// #[test]
// fn copying_file() {
//     let dir = tempdir().unwrap();
//     let p = dir.path().join("burger.txt");
//     let dir = dir.path().join("a");
//     create_dir(&dir).unwrap();

//     create_file(&p).unwrap();
//     fs::write(&p, "Burger").unwrap();
//     copy_file(&p).unwrap();

//     let ctx = ClipboardContext::new().unwrap();
//     let files = ctx.get_files().unwrap();

//     println!("{files:?}");

//     fs::copy(p, dir.join("burger.txt")).unwrap();

//     // assert!(copy_file(p).is_ok());

// }


#[test]
fn system_clipboard() {

    let file_name = "burger.txt";
    if let Ok(var) = fs::exists(file_name){
        if var == false{
            create_file(&file_name).unwrap();
        }
    }
    let absolute_path = fs::canonicalize(file_name).unwrap();
    println!("{absolute_path:?}");

    copy_file(&absolute_path).unwrap();

    let time = std::time::Duration::from_secs(30);

    sleep(time);
}
