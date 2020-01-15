use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KubeConfig {
    pub contexts: Vec<NamedContext>,
    #[serde(rename = "current-context")]
    pub current_context: Option<String>,
    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}

impl KubeConfig {
    pub fn write_to<W: Write>(&self, writer: W) -> anyhow::Result<()> {
        serde_yaml::to_writer(writer, self)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamedContext {
    pub name: String,
    pub context: Context,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub cluster: String,
    pub namespace: String,
    pub user: String,
}

pub fn load(path: impl AsRef<Path>) -> anyhow::Result<KubeConfig> {
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    let obj = serde_yaml::from_reader(reader)?;
    Ok(obj)
}

pub fn get_kubeconfig_path() -> Result<PathBuf> {
    let path = env::var_os("KUBECONFIG").context("KUBECONFIG not found")?;
    Ok(PathBuf::from(path))
}

pub fn get_current_config() -> Result<KubeConfig> {
    load(get_kubeconfig_path()?)
}
