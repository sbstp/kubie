use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context as _, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::settings::Settings;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KubeConfig {
    pub clusters: Vec<NamedCluster>,
    pub users: Vec<NamedUser>,
    pub contexts: Vec<NamedContext>,
    #[serde(rename = "current-context")]
    pub current_context: Option<String>,
    #[serde(flatten)]
    pub others: HashMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamedCluster {
    pub name: String,
    pub cluster: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamedUser {
    pub name: String,
    pub user: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NamedContext {
    pub name: String,
    pub context: Context,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub cluster: String,
    #[serde(default = "default_namespace")]
    pub namespace: String,
    pub user: String,
}

fn default_namespace() -> String {
    "default".to_string()
}

#[derive(Debug)]
pub struct Installed {
    pub clusters: Vec<NamedCluster>,
    pub users: Vec<NamedUser>,
    pub contexts: Vec<NamedContext>,
}

impl KubeConfig {
    pub fn write_to<W: Write>(&self, writer: W) -> anyhow::Result<()> {
        serde_yaml::to_writer(writer, self)?;
        Ok(())
    }
}

impl Installed {
    pub fn find_cluster_by_name(&self, name: &str) -> Option<&NamedCluster> {
        self.clusters.iter().find(|c| c.name == name)
    }

    pub fn find_user_by_name(&self, name: &str) -> Option<&NamedUser> {
        self.users.iter().find(|u| u.name == name)
    }

    pub fn find_contexts_by_cluster(&self, name: &str) -> Vec<&NamedContext> {
        self.contexts.iter().filter(|c| c.context.cluster == name).collect()
    }

    pub fn find_contexts_by_user(&self, name: &str) -> Vec<&NamedContext> {
        self.contexts.iter().filter(|c| c.context.user == name).collect()
    }

    pub fn make_kubeconfig_for_context(&self, context_name: &str, namespace_name: Option<&str>) -> Result<KubeConfig> {
        let mut context = self
            .contexts
            .iter()
            .find(|c| c.name == context_name)
            .cloned()
            .ok_or(anyhow!("Could not find context {}", context_name))?;

        if let Some(namespace_name) = namespace_name {
            context.context.namespace = namespace_name.to_string();
        }

        let cluster = self
            .find_cluster_by_name(&context.context.cluster)
            .cloned()
            .ok_or(anyhow!(
                "Could not find cluster {} referenced by context {}",
                context.context.cluster,
                context_name,
            ))?;

        let user = self.find_user_by_name(&context.context.user).cloned().ok_or(anyhow!(
            "Could not find user {} referenced by context {}",
            context.context.user,
            context_name,
        ))?;

        Ok(KubeConfig {
            clusters: vec![cluster],
            contexts: vec![context],
            users: vec![user],
            current_context: Some(context_name.into()),
            others: {
                let mut m: HashMap<String, Value> = HashMap::new();
                m.insert("apiVersion".into(), Value::String("v1".into()));
                m.insert("kind".into(), Value::String("Config".into()));
                m
            },
        })
    }
}

pub fn get_installed_contexts(settings: &Settings) -> Result<Installed> {
    let mut installed = Installed {
        clusters: vec![],
        contexts: vec![],
        users: vec![],
    };

    for path in settings.get_kube_configs_paths()? {
        match load(&path) {
            Ok(mut kubeconfig) => {
                installed.clusters.extend(kubeconfig.clusters.drain(..));
                installed.contexts.extend(kubeconfig.contexts.drain(..));
                installed.users.extend(kubeconfig.users.drain(..));
            }
            Err(err) => {
                eprintln!("Error loading kubeconfig {}: {}", path.display(), err);
            }
        }
    }

    Ok(installed)
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
