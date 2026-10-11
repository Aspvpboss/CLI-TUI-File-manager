use pfad_core::PfadError;
use std::io::ErrorKind;

pub fn io_kind(err: PfadError) -> ErrorKind {
    match err {
        PfadError::Io(e) => e.kind(),
        other => panic!("expected PfadError::Io, got {other:?}"),
    }
}
