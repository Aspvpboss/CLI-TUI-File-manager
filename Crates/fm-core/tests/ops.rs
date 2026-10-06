
use fm_core::*;
use std::fs;
use tempfile::tempdir;



// create_file

#[test]
fn create_file_makes_an_empty_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");

    create_file(&p).unwrap();

    assert!(p.is_file());
    assert_eq!(fs::metadata(&p).unwrap().len(), 0);
}

#[test]
fn create_file_makes_missing_parent_dirs() {
    let dir = tempdir().unwrap();
    let parent = dir.path().join("one").join("two");
    let p = parent.join("a.txt");

    create_file(&p).unwrap();

    assert!(parent.is_dir());
    assert!(p.is_file());
}

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
fn create_file_reports_the_conflicting_path() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    create_file(&p).unwrap();
 
    match create_file(&p) {
        Err(FmError::AlreadyExists(path)) => assert_eq!(path, p),
        other => panic!("expected AlreadyExists, got {other:?}"),
    }
}
 
#[test]
fn create_file_fails_when_path_is_a_directory() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_dir(&sub).unwrap();
 
    assert!(create_file(&sub).is_err());
    assert!(sub.is_dir());
}
 
#[test]
fn create_file_fails_when_parent_is_a_file() {
    let dir = tempdir().unwrap();
    let blocker = dir.path().join("blocker");
    create_file(&blocker).unwrap();
 
    let result = create_file(blocker.join("a.txt"));
 
    assert!(matches!(result, Err(FmError::Io(_))));
    assert!(blocker.is_file());
}



// rename

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



// remove_dir

#[test]
fn non_recursive_remove_fails_on_full_dir() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_file(sub.join("f.txt")).unwrap();

    assert!(remove_dir(&sub, Recursion::No).is_err());
    assert!(remove_dir(&sub, Recursion::Yes).is_ok());
}