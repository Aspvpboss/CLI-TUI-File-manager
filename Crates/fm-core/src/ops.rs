
use crate::error::{FmError, Result};
use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::{io, result};
use std::path::{Path, PathBuf};

pub enum Recursion {
    Yes,
    No,
}

pub struct Entry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
}




pub fn create_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            Err(FmError::AlreadyExists(path.to_path_buf()))
        }
        Err(e) => Err(e.into()),
    }
}

pub fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if path.exists() {
        return Err(FmError::AlreadyExists(path.to_path_buf()));
    }
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn remove_dir(path: impl AsRef<Path>, recursion: Recursion) -> Result<()> {
    match recursion {
        Recursion::Yes => fs::remove_dir_all(path)?,
        Recursion::No =>  fs::remove_dir(path)?,
    }
    Ok(())
}

pub fn rename(path: impl AsRef<Path>, new_name: impl AsRef<OsStr>) -> Result<PathBuf> {
    let path = path.as_ref();
    let new_name = new_name.as_ref();

    if Path::new(new_name).file_name() != Some(new_name) {
        return Err(FmError::InvalidName(new_name.to_string_lossy().into_owned()));
    }

    let dest = path.with_file_name(new_name);
    if dest.exists() {
        return Err(FmError::AlreadyExists(dest));
    }
    fs::rename(path, &dest)?;
    Ok(dest)
}

// don't even ask me how this works dog.
pub fn list_dir(path: impl AsRef<Path>) -> Result<Vec<Entry>> {
    let mut entries = fs::read_dir(path)?.map(|res| -> Result<Entry> {
        let de = res?;
        let meta = de.metadata()?;
        Ok(Entry {
            name: de.file_name().to_string_lossy().into_owned(),
            is_dir: meta.is_dir(),
            size: meta.len(),
        })
    }).collect::<Result<Vec<_>>>()?;

    entries.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(entries)
}