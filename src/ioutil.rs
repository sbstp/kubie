use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
use fs2::FileExt;
use serde::{de::DeserializeOwned, Serialize};

pub fn read_json<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    let obj = serde_json::from_reader(reader)?;
    Ok(obj)
}

pub fn write_json<P, T>(path: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let path = path.as_ref();
    DirBuilder::new()
        .recursive(true)
        .create(path.parent().expect("path has no parent"))?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, obj)?;
    Ok(())
}

pub fn read_yaml<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    let obj = serde_yaml::from_reader(reader)?;
    Ok(obj)
}

pub fn write_yaml<P, T>(path: P, obj: &T) -> Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let path = path.as_ref();
    DirBuilder::new()
        .recursive(true)
        .create(path.parent().expect("path has no parent"))?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_yaml::to_writer(writer, obj)?;
    Ok(())
}

pub struct LockFile {
    inner: File,
}

pub struct LockFileGuard<'a> {
    lock_file: &'a mut LockFile,
}

impl LockFile {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let inner = OpenOptions::new()
            .append(true)
            .truncate(false)
            .read(true)
            .write(true)
            .create(true)
            .open(path.as_ref())?;
        Ok(LockFile { inner })
    }

    pub fn acquire(&mut self) -> io::Result<LockFileGuard> {
        self.inner.lock_exclusive()?;
        Ok(LockFileGuard { lock_file: self })
    }
}

impl<'a> Drop for LockFileGuard<'a> {
    fn drop(&mut self) {
        let _ = self.lock_file.inner.unlock();
    }
}
