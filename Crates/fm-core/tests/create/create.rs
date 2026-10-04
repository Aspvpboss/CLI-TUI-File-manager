use fm_core::{new, newf};
use std::{env, fs};

#[test]
fn create_file_and_parents() {
    let dir = env::temp_dir().join("new_test");
    let _ = fs::remove_dir_all(&dir);

    let file = dir.join("a/b/c.txt");
    new(file.to_str().unwrap()).unwrap();
    assert!(file.is_file());

    let folder = dir.join("x/y");
    newf(folder.to_str().unwrap()).unwrap();
    assert!(folder.is_dir());

    fs::remove_dir_all(&dir).unwrap();
}