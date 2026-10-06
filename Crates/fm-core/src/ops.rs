use clipboard_rs::{Clipboard, ClipboardContext};
use crate::error::{FmError, Result};
use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::fmt;
use std::sync::{LazyLock, Mutex};

// black magic, thread safe though
static CLIPBOARD: LazyLock<Option<Mutex<ClipboardContext>>> =
    LazyLock::new(|| ClipboardContext::new().ok().map(Mutex::new));

pub enum Recursion {
    Yes,
    No,
}

pub enum PasteDeleteRef {
    Yes,
    No,
}


#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut name = self.name.clone();
        if self.is_dir == true {
            name += "/";
        }
        write!(f, "{} - {} bytes", name, self.size)
    }
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


pub fn copy_files(paths : Vec<impl AsRef<Path>>) -> Result<()> {

    let mut path_strings: Vec<String> = Vec::new();

    // do not question this
    for path in paths {
        let path_string = path.as_ref().to_str().unwrap_or("").to_string();
        path_strings.push(path_string);
    }

    
    // do NOT question this
    let Ok(safe_clipboard) = CLIPBOARD.as_ref().ok_or(FmError::Clipboard(String::from("Failed to get clipboard ref")))?.lock() else {
        return Err(FmError::Clipboard(String::from("Failed to get clipboard ref")));
    };


    if path_strings.is_empty() {
        if safe_clipboard.clear().is_err() {
            return Err(FmError::Clipboard(String::from("Failed to clear clipboard")));
        }
        return Ok(());
    }

    if let Err(_) = safe_clipboard.set_files(path_strings){
        return Err(FmError::Clipboard(String::from("Failed to copy file to clipboard")));
    }

    Ok(())
}

pub fn paste_files(delete_reference : PasteDeleteRef) -> Result<()> {

    let Ok(safe_clipboard) = CLIPBOARD.as_ref().ok_or(FmError::Clipboard(String::from("Failed to get clipboard ref")))?.lock() else {
        return Err(FmError::Clipboard(String::from("Failed to get clipboard ref")));
    };

    // An empty clipboard makes get_files() error on Windows; treat it as "nothing to paste"
    let clipboard_files = safe_clipboard.get_files().unwrap_or_default();

    for file in &clipboard_files {
        create_file(file)?;
    }

    match delete_reference {
        PasteDeleteRef::Yes => {
            for file in &clipboard_files {
                remove_file(file)?;
            }
        },
        PasteDeleteRef::No => {},
    } 

    Ok(())
}


