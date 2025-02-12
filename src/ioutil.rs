use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::{
    fs::{DirBuilder, File, OpenOptions},
    panic::{self, UnwindSafe},
};

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
    F: FnOnce() -> Result<T, anyhow::Error> + UnwindSafe,
{
    let path = path.as_ref();
    let file = OpenOptions::new()
        .append(true)
        .truncate(false)
        .read(true)
        .create(true)
        .open(path)
        .with_context(|| format!("Could not open lock file at {}", path.display()))?;

    file.lock_exclusive()
        .with_context(|| format!("Could not lock file at {}", path.display()))?;

    // Run the given closure, but catch any panics so we can unlock before resuming the panic.
    let exception = panic::catch_unwind(scope);

    // Ignore errors during unlock. If we had a panic, we don't want to return the potential error.
    // If we did not panic, we want to return the closure's result.
    let _ = FileExt::unlock(&file);

    match exception {
        Ok(result) => result,
        Err(x) => panic::resume_unwind(x),
    }
}
