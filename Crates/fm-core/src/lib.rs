
use std::fs::{self, File};
use std::io;
use std::path::Path;

// Placeholder so the others have something to call, and so I can actually build something.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// All of these are super super simple versions of what their final versions should be.
// They all still need conflic checks and whatnot. Also all delete functions should probably
// have recursive flags so you don't always recursively delete. Probably.

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

pub fn del(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

pub fn delf(path: &str) -> io::Result<()> {
    fs::remove_dir_all(path)
}

pub fn rename(path: &str, name: &str) -> io::Result<()> {
    fs::rename(path, name)
}