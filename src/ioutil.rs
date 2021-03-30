use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::{Context, Result};
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

pub fn file_lock<P, F, T>(path: P, scope: F) -> Result<T, anyhow::Error>
where
    P: AsRef<Path>,
    F: FnOnce() -> Result<T, anyhow::Error>,
{
    let path = path.as_ref();
    let file = OpenOptions::new()
        .append(true)
        .truncate(false)
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .with_context(|| format!("Could not open lock file at {}", path.display()))?;

    file.lock_exclusive()
        .with_context(|| format!("Could not lock file at {}", path.display()))?;

    let result = scope();

    file.unlock()
        .with_context(|| format!("Could not unlock file at {}", path.display()))?;

    result
}
