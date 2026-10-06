use fm_core::FmError;
use std::error::Error;
use std::io;
use std::path::PathBuf;



#[test]
fn io_error_converts_to_fm_error() {
    let err: FmError = io::Error::new(io::ErrorKind::NotFound, "gone").into();

    match err {
        FmError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
        other => panic!("expected FmError::Io, got {other:?}"),
    }
}

#[test]
fn question_mark_converts_io_errors() {
    fn read_missing() -> fm_core::Result<String> {
        Ok(std::fs::read_to_string("this/path/does/not/exist")?)
    }

    assert!(matches!(read_missing(), Err(FmError::Io(_))));
}

#[test]
fn only_io_errors_have_a_source() {
    let io_err = FmError::Io(io::Error::other("boom"));
    let exists = FmError::AlreadyExists(PathBuf::from("a.txt"));
    let invalid = FmError::InvalidName("../x".to_string());

    assert!(io_err.source().is_some());
    assert!(exists.source().is_none());
    assert!(invalid.source().is_none());
}

#[test]
fn messages_name_the_thing_that_went_wrong() {
    let io_err = FmError::Io(io::Error::other("boom"));
    let exists = FmError::AlreadyExists(PathBuf::from("a.txt"));
    let invalid = FmError::InvalidName("../x".to_string());

    assert!(exists.to_string().contains("a.txt"));
    assert!(invalid.to_string().contains("../x"));
    assert!(io_err.to_string().contains("boom"));
}