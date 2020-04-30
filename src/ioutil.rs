use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;
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
    let file = File::create(path.as_ref())?;
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
    let file = File::create(path.as_ref())?;
    let writer = BufWriter::new(file);
    serde_yaml::to_writer(writer, obj)?;
    Ok(())
}
