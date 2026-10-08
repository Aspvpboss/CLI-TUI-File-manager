
use clipboard_rs::{Clipboard, ClipboardContext};
use crate::error::{FmError, Result};
use std::ffi::OsStr;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};

// black magic, thread safe though
static CLIPBOARD: LazyLock<Option<Mutex<ClipboardContext>>> =
    LazyLock::new(|| ClipboardContext::new().ok().map(Mutex::new));
// BIG IMPORTANT ! ! ! This is were copied paths go whe the system clipboard can't take them/does not exists.
static FALLBACK: Mutex<Option<Vec<PathBuf>>> = Mutex::new(None);
// Remember if the clipboard can't be used
static CLIPBOARD_DISABLED: AtomicBool = AtomicBool::new(false);

pub enum Recursion {
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
        if self.is_dir {
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

pub fn copy_files(paths: Vec<impl AsRef<Path>>) -> Result<()> {
    let mut absolute_paths: Vec<PathBuf> = Vec::new();
    for path in &paths {
        absolute_paths.push(std::path::absolute(path)?);
    }

    let Ok(mut fallback) = FALLBACK.lock() else {
        return Err(FmError::Clipboard(String::from("fallback clipboard lock poisoned")))
    };
    
    if copy_to_system(&absolute_paths) {
        fallback.None();
    } else {
        *fallback = Some(absolute_paths);
    }

    Ok(())
}
/// Internal function for copy_files()
fn copy_to_system(paths: &[PathBuf]) -> bool {
    if CLIPBOARD_DISABLED.load(Ordering::Relaxed) {
        return false;
    }

    let Some(clipboard) = CLIPBOARD.as_ref() else {
        return false;
    };
    let Ok(clipboard) = clipboard.lock() else {
        CLIPBOARD_DISABLED.store(true, Ordering::Relaxed);
        return false;
    };

    let mut path_strings: Vec<String> = Vec::new();
    for path in paths {
        let Some(text) = path.to_str() else {
            return false;
        };
        path_strings.push(text.to_string());
    }

    let ok = if path_strings.is_empty() {
        clipboard.clear().is_ok()
    } else {
        clipboard.set_files(path_strings).is_ok()
    };

    if !ok {
        CLIPBOARD_DISABLED.store(true, Ordering::Relaxed);
    }

    ok
}

pub fn paste_files(dest_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let dest_dir = std::path::absolute(dest_dir)?;
    let mut created = Vec::new();

    for src in clipboard_paths()? {
        let Some(name) = src.file_name() else { continue };
        let dest = dest_dir.join(name);

        if dest.exists() {
            return Err(FmError::AlreadyExists(dest));
        }

        if src.is_dir() {
            if dest_dir.starts_with(&src) {
                return Err(FmError::PasteIntoSelf(src));
            }
            copy_dir_all(&src, &dest)?;
        } else {
            fs::copy(&src, &dest)?;
        }
        created.push(dest);
    }

    Ok(created)
}

fn copy_dir_all(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir(dest)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let target = dest.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }

    Ok(())
}

fn clipboard_paths() -> Result<Vec<PathBuf>> {
    let Ok(fallback) = FALLBACK.lock() else {
        return Err(FmError::Clipboard(String::from("fallback clipboard lock poisoned")));
    };

    if let Some(paths) = fallback.as_ref() {
        return Ok(paths.clone())
    }

    let Some(clipboard) = CLIPBOARD.as_ref() else {
        return Ok(Vec::new());
    };
    let Ok(clipboard) = clipboard.lock() else {
        return Ok(Vec::new());
    };

    let files = clipboard.get_files().unwrap_or_default();

    let mut paths = Vec::new();
    for file in files {
        // Linux returns file://home/.. while windows returns a plain path
        let text = file.strip_prefix("file://").unwrap_or(file.as_str());
        paths.push(PathBuf::from(text));
    }

    Ok(paths)
}