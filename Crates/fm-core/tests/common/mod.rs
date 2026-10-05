use std::path::PathBuf;
use std::{env, fs};

/// Fresh, empty scratch directory unique to this test, so tests can run in parallel.
pub fn scratch(name: &str) -> PathBuf {
    let dir = env::temp_dir().join(format!("fm_core_{}_{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}
