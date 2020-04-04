use std::fs::{self, Permissions, File};
use std::os::unix::prelude::*;
use std::ops::{Deref, DerefMut, Drop};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

/// Tempfile is an object which generates a random name for a file and opens it for editing.
///
/// When the object is dropped, the file is closed and unlinked.
pub struct Tempfile {
    file: Option<File>,
    path: PathBuf,
}

impl Tempfile {
    pub fn new(base_dir: impl AsRef<Path>, prefix: impl AsRef<str>, suffix: impl AsRef<str>) -> Result<Tempfile> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is before unix epoch");
        let nanos = duration.as_nanos();
        let filename = format!("{}{}{}", prefix.as_ref(), nanos, suffix.as_ref());
        let path = base_dir.as_ref().join(filename);
        let file = File::create(&path)?;
        fs::set_permissions(&path, Permissions::from_mode(0o600))?;
        Ok(Tempfile {
            file: Some(file),
            path: path,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Deref for Tempfile {
    type Target = File;

    fn deref(&self) -> &File {
        self.file.as_ref().unwrap()
    }
}

impl DerefMut for Tempfile {
    fn deref_mut(&mut self) -> &mut File {
        self.file.as_mut().unwrap()
    }
}

impl Drop for Tempfile {
    fn drop(&mut self) {
        drop(self.file.take().unwrap());
        let _ = fs::remove_file(&self.path);
    }
}
