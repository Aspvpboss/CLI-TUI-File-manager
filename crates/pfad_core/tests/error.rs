
use pfad_core::PfadError;
use std::error::Error;
use std::io;
use std::path::PathBuf;



#[test]
fn io_error_converts_to_pfad_error() {
    let err: PfadError = io::Error::new(io::ErrorKind::NotFound, "gone").into();

    match err {
        PfadError::Io(e) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
        other => panic!("expected PfadError::Io, got {other:?}"),
    }
}

#[test]
fn question_mark_converts_io_errors() {
    fn read_missing() -> pfad_core::Result<String> {
        Ok(std::fs::read_to_string("this/path/does/not/exist")?)
    }

    assert!(matches!(read_missing(), Err(PfadError::Io(_))));
}

#[test]
fn only_io_errors_have_a_source() {
    let io_err = PfadError::Io(io::Error::other("boom"));
    let exists = PfadError::AlreadyExists(PathBuf::from("a.txt"));
    let invalid = PfadError::InvalidName("../x".to_string());

    assert!(io_err.source().is_some());
    assert!(exists.source().is_none());
    assert!(invalid.source().is_none());
}

#[test]
fn messages_name_the_thing_that_went_wrong() {
    let io_err = PfadError::Io(io::Error::other("boom"));
    let exists = PfadError::AlreadyExists(PathBuf::from("a.txt"));
    let invalid = PfadError::InvalidName("../x".to_string());

    assert!(exists.to_string().contains("a.txt"));
    assert!(invalid.to_string().contains("../x"));
    assert!(io_err.to_string().contains("boom"));
}