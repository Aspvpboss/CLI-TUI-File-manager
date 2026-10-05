
use std::{fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum FmError {
    Cli(String),
    Io(io::Error),
    AlreadyExists(PathBuf),
    InvalidName(String),
}

impl fmt::Display for FmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FmError::Io(e)  => write!(f, "I/O error: {e}"),
            FmError::AlreadyExists(p) => write!(f, "alredy exists: {}", p.display()),
            FmError::InvalidName(n) => write!(f, "invalid name: {n:?}"),
            FmError::Cli(n) => write!(f, "failed to CLI args: {n:?}"),
        }
    }
}

impl std::error::Error for FmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FmError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for FmError {
    fn from(e: io::Error) -> Self {
        FmError::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, FmError>;