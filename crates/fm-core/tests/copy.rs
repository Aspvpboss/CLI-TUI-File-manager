
use fm_core::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use tempfile::tempdir;

#[test]
fn copy_single_file_succeeds() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "one").unwrap();

    assert!(copy_files(vec![&file]).is_ok());
}

#[test]
fn copy_multiple_files_succeeds() {
    let dir = tempdir().unwrap();
    let paths: Vec<PathBuf> = ["a.txt", "b.txt", "c.txt"]
        .iter()
        .map(|n| dir.path().join(n))
        .collect();
    for p in &paths {
        fs::write(p, "x").unwrap();
    }

    assert!(copy_files(paths).is_ok());
}

#[test]
fn copy_directory_succeeds() {
    let dir = tempdir().unwrap();
    let sub = dir.path().join("sub");
    fs::create_dir_all(sub.join("nested")).unwrap();
    fs::write(sub.join("nested").join("f.txt"), "x").unwrap();

    assert!(copy_files(vec![&sub]).is_ok());
}

#[test]
fn copy_mixed_files_and_dirs_succeeds() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let sub = dir.path().join("sub");
    fs::write(&file, "x").unwrap();
    fs::create_dir(&sub).unwrap();

    assert!(copy_files(vec![file, sub]).is_ok());
}

#[test]
fn copy_empty_list_succeeds() {
    // Empty input clears the clipboard (or fallback) instead of erroring.
    assert!(copy_files(Vec::<PathBuf>::new()).is_ok());
}

#[test]
fn copy_empty_list_after_copy_succeeds() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    assert!(copy_files(vec![&file]).is_ok());
    assert!(copy_files(Vec::<PathBuf>::new()).is_ok());
}

#[test]
fn copy_accepts_relative_path() {
    // Relative paths are made absolute against the current directory; the
    // file does not need to exist for that.
    assert!(copy_files(vec!["some_relative_file.txt"]).is_ok());
    assert!(copy_files(vec!["./some/relative/dir"]).is_ok());
    assert!(copy_files(vec!["../up_one_level.txt"]).is_ok());
}

#[test]
fn copy_accepts_relative_path_to_existing_file() {
    // Cargo runs integration tests with the package root as the cwd.
    assert!(Path::new("Cargo.toml").exists());
    assert!(copy_files(vec!["Cargo.toml"]).is_ok());
}

#[test]
fn copy_missing_path_is_accepted() {
    // Existence is never checked, and nothing gets created as a side effect.
    let dir = tempdir().unwrap();
    let missing = dir.path().join("does_not_exist");

    assert!(copy_files(vec![&missing]).is_ok());
    assert!(!missing.exists());
}

#[test]
fn copy_mixed_existing_and_missing_paths() {
    let dir = tempdir().unwrap();
    let real = dir.path().join("real.txt");
    let missing = dir.path().join("missing.txt");
    fs::write(&real, "x").unwrap();

    assert!(copy_files(vec![&real, &missing]).is_ok());
    assert!(real.exists());
    assert!(!missing.exists());
}

#[test]
fn copy_accepts_str_pathbuf_and_path_refs() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    let as_str = file.to_str().unwrap();
    assert!(copy_files(vec![as_str]).is_ok());
    assert!(copy_files(vec![String::from(as_str)]).is_ok());
    assert!(copy_files(vec![file.clone()]).is_ok());
    assert!(copy_files(vec![file.as_path()]).is_ok());
    assert!(copy_files(Vec::<&Path>::new()).is_ok());
}

#[test]
fn copy_duplicate_paths_succeeds() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    assert!(copy_files(vec![&file, &file, &file]).is_ok());
}

#[test]
fn copy_path_with_spaces_and_unicode_succeeds() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("my file – ünïcödé 文件.txt");
    fs::write(&file, "x").unwrap();

    assert!(copy_files(vec![&file]).is_ok());
}

#[test]
fn copy_does_not_modify_source() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    let sub = dir.path().join("sub");
    fs::write(&file, "one").unwrap();
    fs::create_dir(&sub).unwrap();
    fs::write(sub.join("inner.txt"), "inner").unwrap();

    copy_files(vec![&file, &sub]).unwrap();

    assert_eq!(fs::read_to_string(&file).unwrap(), "one");
    assert!(sub.is_dir());
    assert_eq!(fs::read_to_string(sub.join("inner.txt")).unwrap(), "inner");
}

#[test]
fn copy_does_not_create_copies_next_to_source() {
    // copy_files only records paths; it must not duplicate any data on disk.
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    copy_files(vec![&file]).unwrap();

    let count = fs::read_dir(dir.path()).unwrap().count();
    assert_eq!(count, 1);
}

#[test]
fn copy_twice_succeeds() {
    // A second call replaces the first selection (clipboard or fallback),
    // including after the clipboard was cleared by an empty list.
    let dir = tempdir().unwrap();
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    fs::write(&a, "a").unwrap();
    fs::write(&b, "b").unwrap();

    assert!(copy_files(vec![&a]).is_ok());
    assert!(copy_files(vec![&b]).is_ok());
    assert!(copy_files(Vec::<PathBuf>::new()).is_ok());
    assert!(copy_files(vec![&a, &b]).is_ok());
}

#[test]
fn copy_keeps_succeeding_when_clipboard_is_unavailable() {
    // If the clipboard is missing or fails once, it is remembered as disabled
    // and every later call goes straight to the fallback list. Whatever the
    // environment is, repeated calls must keep returning Ok.
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    for _ in 0..20 {
        assert!(copy_files(vec![&file]).is_ok());
        assert!(copy_files(Vec::<PathBuf>::new()).is_ok());
    }
}

#[cfg(unix)]
#[test]
fn copy_non_utf8_path_falls_back_and_succeeds() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let dir = tempdir().unwrap();
    let weird = dir.path().join(OsStr::from_bytes(b"bad\xff\xfename"));
    fs::write(&weird, "x").unwrap();

    assert!(copy_files(vec![&weird]).is_ok());
    // A later, valid copy must still work.
    assert!(copy_files(vec![dir.path()]).is_ok());
}

#[cfg(windows)]
#[test]
fn copy_non_utf8_path_falls_back_and_succeeds() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    // An unpaired surrogate is valid in a Windows path but is not valid UTF-8,
    // so `to_str()` fails and copy_to_system must report false.
    let dir = tempdir().unwrap();
    let weird = dir.path().join(OsString::from_wide(&[0x62, 0x61, 0x64, 0xD800]));

    assert!(copy_files(vec![&weird]).is_ok());
    // A later, valid copy must still work.
    assert!(copy_files(vec![dir.path()]).is_ok());
}

#[test]
fn copy_is_safe_across_threads() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("a.txt");
    fs::write(&file, "x").unwrap();

    let handles: Vec<_> = (0..8)
        .map(|i| {
            let file = file.clone();
            thread::spawn(move || {
                for j in 0..10 {
                    // Mix real copies with clears so the clipboard lock and the
                    // fallback mutex are both contended.
                    if (i + j) % 3 == 0 {
                        copy_files(Vec::<PathBuf>::new()).unwrap();
                    } else {
                        copy_files(vec![&file]).unwrap();
                    }
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
