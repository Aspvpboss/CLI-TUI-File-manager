use fm_core::FmError;
use std::io::ErrorKind;

pub fn io_kind(err: FmError) -> ErrorKind {
    match err {
        FmError::Io(e) => e.kind(),
        other => panic!("expected FmError::Io, got {other:?}"),
    }
}
