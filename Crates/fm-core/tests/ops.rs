
// Claude is responsible for most tests in this file past the creat_file tests

use fm_core::*;
use std::fs;
use std::io::ErrorKind;
use tempfile::tempdir;

fn io_kind(err: FmError) -> ErrorKind {
    match err {
        FmError::Io(e) => e.kind(),
        other => panic!("expected FmError::Io, got {other:?}"),
    }
}



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



// create_dir

#[test]
fn create_dir_makes_a_directory() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
 
    create_dir(&sub).unwrap();
 
    assert!(sub.is_dir());
}
 
#[test]
fn create_dir_makes_nested_directories() {
    let dir = tempdir().unwrap();
    let deep = dir.path().join("one").join("two").join("three");
 
    create_dir(&deep).unwrap();
 
    assert!(deep.is_dir());
}
 
#[test]
fn create_dir_refuses_existing_directory() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    let inner = sub.join("f.txt");
    create_file(&inner).unwrap();
 
    match create_dir(&sub) {
        Err(FmError::AlreadyExists(path)) => assert_eq!(path, sub),
        other => panic!("expected AlreadyExists, got {other:?}"),
    }
    assert!(inner.is_file());
}
 
#[test]
fn create_dir_refuses_existing_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("taken");
    fs::write(&p, "hello").unwrap();
 
    assert!(matches!(create_dir(&p), Err(FmError::AlreadyExists(_))));
    assert!(p.is_file());
    assert_eq!(fs::read_to_string(&p).unwrap(), "hello");
}



// remove_file

#[test]
fn remove_file_deletes_only_that_file() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    create_file(&a).unwrap();
    create_file(&b).unwrap();
 
    remove_file(&a).unwrap();
 
    assert!(!a.exists());
    assert!(b.is_file());
}
 
#[test]
fn remove_file_missing_is_not_found() {
    let dir = tempdir().unwrap();
 
    let err = remove_file(dir.path().join("nope.txt")).unwrap_err();
 
    assert_eq!(io_kind(err), ErrorKind::NotFound);
}
 
#[test]
fn remove_file_refuses_a_directory() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_dir(&sub).unwrap();
 
    assert!(matches!(remove_file(&sub), Err(FmError::Io(_))));
    assert!(sub.is_dir());
}



// remove_dir

#[test]
fn remove_dir_non_recursive_removes_empty_dir() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_dir(&sub).unwrap();
 
    remove_dir(&sub, Recursion::No).unwrap();
 
    assert!(!sub.exists());
}
 
#[test]
fn non_recursive_remove_fails_on_full_dir() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    create_file(sub.join("f.txt")).unwrap();
 
    assert!(remove_dir(&sub, Recursion::No).is_err());
    assert!(remove_dir(&sub, Recursion::Yes).is_ok());
}
 
#[test]
fn failed_non_recursive_remove_leaves_contents() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    let inner = sub.join("f.txt");
    fs::create_dir(&sub).unwrap();
    fs::write(&inner, "hello").unwrap();
 
    assert!(remove_dir(&sub, Recursion::No).is_err());
 
    assert!(sub.is_dir());
    assert_eq!(fs::read_to_string(&inner).unwrap(), "hello");
}
 
#[test]
fn remove_dir_recursive_removes_whole_tree() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    let sibling = dir.path().join("sibling.txt");
    create_file(sub.join("a").join("b").join("deep.txt")).unwrap();
    create_file(sub.join("top.txt")).unwrap();
    create_file(&sibling).unwrap();
 
    remove_dir(&sub, Recursion::Yes).unwrap();
 
    assert!(!sub.exists());
    assert!(sibling.is_file());
}
 
#[test]
fn remove_dir_missing_is_not_found() {
    let dir = tempdir().unwrap();
    let missing = dir.path().join("nope");
 
    for recursion in [Recursion::No, Recursion::Yes] {
        let err = remove_dir(&missing, recursion).unwrap_err();
        assert_eq!(io_kind(err), ErrorKind::NotFound);
    }
}
 
#[test]
fn remove_dir_refuses_a_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    fs::write(&p, "hello").unwrap();
 
    for recursion in [Recursion::No, Recursion::Yes] {
        assert!(matches!(remove_dir(&p, recursion), Err(FmError::Io(_))));
    }
    assert_eq!(fs::read_to_string(&p).unwrap(), "hello");
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
    assert!(matches!(
        rename(&new, "../escape.txt"),
        Err(FmError::InvalidName(_))
    ));
}
 
#[test]
fn rename_keeps_file_contents() {
    let dir = tempdir().unwrap();
    let old = dir.path().join("old.txt");
    fs::write(&old, "hello").unwrap();
 
    let new = rename(&old, "new.txt").unwrap();
 
    assert_eq!(fs::read_to_string(&new).unwrap(), "hello");
}
 
#[test]
fn rename_directory_keeps_its_children() {
    let dir = tempdir().unwrap();
    let old = dir.path().join("old");
    fs::create_dir(&old).unwrap();
    fs::write(old.join("f.txt"), "hello").unwrap();
 
    let new = rename(&old, "new").unwrap();
 
    assert_eq!(new, dir.path().join("new"));
    assert!(!old.exists());
    assert_eq!(fs::read_to_string(new.join("f.txt")).unwrap(), "hello");
}
 
#[test]
fn rename_rejects_names_that_are_not_plain_names() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    create_file(&p).unwrap();
 
    for bad in ["", ".", "..", "sub/x.txt", "../x.txt", "/abs.txt", "x/"] {
        match rename(&p, bad) {
            Err(FmError::InvalidName(name)) => assert_eq!(name, bad),
            other => panic!("expected InvalidName for {bad:?}, got {other:?}"),
        }
    }
    assert!(p.is_file());
}
 
#[test]
fn rename_refuses_to_overwrite_a_file() {
    let dir = tempdir().unwrap();
    let one = dir.path().join("one.txt");
    let two = dir.path().join("two.txt");
    fs::write(&one, "one").unwrap();
    fs::write(&two, "two").unwrap();
 
    match rename(&one, "two.txt") {
        Err(FmError::AlreadyExists(path)) => assert_eq!(path, two),
        other => panic!("expected AlreadyExists, got {other:?}"),
    }
    assert_eq!(fs::read_to_string(&one).unwrap(), "one");
    assert_eq!(fs::read_to_string(&two).unwrap(), "two");
}
 
#[test]
fn rename_refuses_to_overwrite_a_directory() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    let sub = dir.path().join("sub");
    create_file(&p).unwrap();
    create_dir(&sub).unwrap();
 
    assert!(matches!(rename(&p, "sub"), Err(FmError::AlreadyExists(_))));
    assert!(p.is_file());
    assert!(sub.is_dir());
}
 
#[test]
fn rename_missing_source_is_not_found() {
    let dir = tempdir().unwrap();
 
    let err = rename(dir.path().join("nope.txt"), "new.txt").unwrap_err();
 
    assert_eq!(io_kind(err), ErrorKind::NotFound);
    assert!(!dir.path().join("new.txt").exists());
}



// list_dir

fn names(entries: &[Entry]) -> Vec<&str> {
    entries.iter().map(|e| e.name.as_str()).collect()
}
 
#[test]
fn list_dir_of_empty_directory_is_empty() {
    let dir = tempdir().unwrap();
 
    assert!(list_dir(dir.path()).unwrap().is_empty());
}
 
#[test]
fn list_dir_reports_name_kind_and_size() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.txt"), "hello").unwrap();
    fs::create_dir(dir.path().join("sub")).unwrap();
 
    let entries = list_dir(dir.path()).unwrap();
 
    assert_eq!(entries.len(), 2);
    let file = entries.iter().find(|e| e.name == "a.txt").unwrap();
    assert!(!file.is_dir);
    assert_eq!(file.size, 5);
    let sub = entries.iter().find(|e| e.name == "sub").unwrap();
    assert!(sub.is_dir);
}
 
#[test]
fn list_dir_sorts_dirs_first_then_by_name_ignoring_case() {
    let dir = tempdir().unwrap();
    create_file(dir.path().join("b.txt")).unwrap();
    create_file(dir.path().join("A.txt")).unwrap();
    create_file(dir.path().join("c.txt")).unwrap();
    create_dir(dir.path().join("Zeta")).unwrap();
    create_dir(dir.path().join("alpha")).unwrap();
 
    let entries = list_dir(dir.path()).unwrap();
 
    assert_eq!(
        names(&entries),
        ["alpha", "Zeta", "A.txt", "b.txt", "c.txt"]
    );
}
 
#[test]
fn list_dir_is_not_recursive() {
    let dir = tempdir().unwrap();
    create_file(dir.path().join("sub").join("inner.txt")).unwrap();
 
    let entries = list_dir(dir.path()).unwrap();
 
    assert_eq!(names(&entries), ["sub"]);
}
 
#[test]
fn list_dir_includes_dotfiles() {
    let dir = tempdir().unwrap();
    create_file(dir.path().join(".hidden")).unwrap();
    create_file(dir.path().join("shown.txt")).unwrap();
 
    let entries = list_dir(dir.path()).unwrap();
 
    assert_eq!(names(&entries), [".hidden", "shown.txt"]);
}
 
#[test]
fn list_dir_missing_is_not_found() {
    let dir = tempdir().unwrap();
 
    let err = list_dir(dir.path().join("nope")).unwrap_err();
 
    assert_eq!(io_kind(err), ErrorKind::NotFound);
}
 
#[test]
fn list_dir_refuses_a_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("a.txt");
    create_file(&p).unwrap();
 
    assert!(matches!(list_dir(&p), Err(FmError::Io(_))));
}



// Entry

#[test]
fn entry_display_marks_directories_with_a_slash() {
    let file = Entry {
        name: "a.txt".to_string(),
        size: 5,
        is_dir: false,
    };
    let sub = Entry {
        name: "sub".to_string(),
        size: 0,
        is_dir: true,
    };
 
    assert_eq!(file.to_string(), "a.txt - 5 bytes");
    assert_eq!(sub.to_string(), "sub/ - 0 bytes");
}