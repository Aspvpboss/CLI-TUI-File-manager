
use fm_core::*;
use tempfile::tempdir;

#[test]
fn create_file_refuses_to_clobber() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");

    create_file(&p).unwrap();
    std::fs::write(&p, "hello").unwrap();

    assert!(matches!(create_file(&p), Err(FmError::AlreadyExists(_))));
    assert_eq!(std::fs::read_to_string(&p).unwrap(), "hello");
}

#[test]
fn rename_stays_in_same_dir() {
    let dir = tempdir().unwrap();
    let old = dir.path().join("old.txt");
    create_file(&old).unwrap();

    let new = rename(&old, "new.txt").unwrap();

    assert_eq!(new, dir.path().join("new.txt"));
    assert!(!old.exists());
    assert!(matches!(rename(&new, "../escape.txt"), Err(FmError::InvalidName(_))));
}

#[test]
fn non_recursive_remove_fails_on_full_dir() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_file(sub.join("f.txt")).unwrap();

    assert!(remove_dir(&sub, Recursion::No).is_err());
    assert!(remove_dir(&sub, Recursion::Yes).is_ok());
}