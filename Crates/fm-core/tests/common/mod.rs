use std::path::PathBuf;
use std::{env, fs};

/// Empty scratch directory, so tests can run in parallel ong!
pub fn scratch(name: &str) -> PathBuf {
    let dir = env::temp_dir().join(format!("fm_core_{}_{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}
