
use std::fs::{self, File};
use std::io;
use std::path::Path;

// Placeholder so the others have something to call, and so I can actually build something.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn new(path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?;
    Ok(())
}

pub fn newf(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}