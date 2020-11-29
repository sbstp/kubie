use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::{anyhow, bail, Context as _, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use wildmatch::WildMatch;

use crate::ioutil;
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
    pub namespace: Option<String>,
    pub user: String,
}

#[derive(Clone, Debug)]
pub struct Sourced<T> {
    pub source: Rc<PathBuf>,
    pub item: T,
}

impl<T> Sourced<T> {
    pub fn new(source: &Rc<PathBuf>, item: T) -> Self {
        Sourced {
            source: source.clone(),
            item,
        }
    }
}

#[derive(Debug)]
pub struct Installed {
    pub clusters: Vec<Sourced<NamedCluster>>,
    pub users: Vec<Sourced<NamedUser>>,
    pub contexts: Vec<Sourced<NamedContext>>,
}

impl KubeConfig {
    pub fn write_to<W: Write>(&self, writer: W) -> anyhow::Result<()> {
        serde_yaml::to_writer(writer, self)?;
        Ok(())
    }
}

impl Installed {
    pub fn find_context_by_name(&self, name: &str) -> Option<&Sourced<NamedContext>> {
        self.contexts.iter().find(|s| s.item.name == name)
    }

    pub fn find_cluster_by_name(&self, name: &str, source: &Path) -> Option<&Sourced<NamedCluster>> {
        self.clusters
            .iter()
            .find(|s| s.item.name == name && *s.source == source)
    }

    pub fn find_user_by_name(&self, name: &str, source: &Path) -> Option<&Sourced<NamedUser>> {
        self.users.iter().find(|s| s.item.name == name && *s.source == source)
    }

    pub fn find_contexts_by_cluster(&self, name: &str, source: &Path) -> Vec<&Sourced<NamedContext>> {
        self.contexts
            .iter()
            .filter(|s| s.item.context.cluster == name && *s.source == source)
            .collect()
    }

    pub fn find_contexts_by_user(&self, name: &str, source: &Path) -> Vec<&Sourced<NamedContext>> {
        self.contexts
            .iter()
            .filter(|s| s.item.context.user == name && *s.source == source)
            .collect()
    }

    pub fn get_contexts_matching(&self, pattern: &str) -> Vec<&Sourced<NamedContext>> {
        let matcher = WildMatch::new(pattern);
        self.contexts
            .iter()
            .filter(|s| matcher.is_match(&s.item.name))
            .collect()
    }

    pub fn delete_context(&mut self, name: &str) -> Result<()> {
        let context = self
            .find_context_by_name(name)
            .ok_or_else(|| anyhow!("Context not found"))?;

        let mut kubeconfig: KubeConfig = ioutil::read_yaml(context.source.as_ref())?;

        // Retain all contexts whose name is not our context.
        kubeconfig.contexts.retain(|x| x.name != context.item.name);

        // Retain all clusters whose name is not our context's cluster reference.
        kubeconfig.clusters.retain(|x| x.name != context.item.context.cluster);

        // Retain all users whose name is not our context's user reference.
        kubeconfig.users.retain(|x| x.name != context.item.context.user);

        if kubeconfig.contexts.is_empty() && kubeconfig.clusters.is_empty() && kubeconfig.users.is_empty() {
            // If the kubeconfig is empty after removing the context and dangling references,
            // we simply remove the file.
            println!(
                "Deleting kubeconfig {} because is it now empty.",
                context.source.display()
            );

            fs::remove_file(context.source.as_ref()).context("Could not delete empty kubeconfig file")?;
        } else {
            // If the kubeconfig is not empty, we rewrite it with the context and dangling references removed.
            println!("Updating kubeconfig {}.", context.source.display());

            ioutil::write_yaml(context.source.as_ref(), &kubeconfig)
                .context("Could not open kubeconfig file to rewrite it.")?;
        }

        Ok(())
    }

    pub fn make_kubeconfig_for_context(
        &self,
        context_name: &str,
        namespace_name: Option<impl Into<String>>,
    ) -> Result<KubeConfig> {
        let mut context_src = self
            .contexts
            .iter()
            .find(|c| c.item.name == context_name)
            .cloned()
            .ok_or(anyhow!("Could not find context {}", context_name))?;

        context_src.item.context.namespace = namespace_name.map(Into::into);

        let cluster = self
            .find_cluster_by_name(&context_src.item.context.cluster, &context_src.source)
            .cloned()
            .ok_or(anyhow!(
                "Could not find cluster {} referenced by context {}",
                context_src.item.context.cluster,
                context_name,
            ))?;

        let user = self
            .find_user_by_name(&context_src.item.context.user, &context_src.source)
            .cloned()
            .ok_or(anyhow!(
                "Could not find user {} referenced by context {}",
                context_src.item.context.user,
                context_name,
            ))?;

        Ok(KubeConfig {
            clusters: vec![cluster.item],
            contexts: vec![context_src.item],
            users: vec![user.item],
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

fn load_kubeconfigs<I, P>(kubeconfigs: I) -> Result<Installed>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut installed = Installed {
        clusters: vec![],
        contexts: vec![],
        users: vec![],
    };

    for path in kubeconfigs.into_iter() {
        let path = path.as_ref();
        let kubeconfig: Result<KubeConfig> = ioutil::read_yaml(&path);

        match kubeconfig {
            Ok(mut kubeconfig) => {
                let path = Rc::new(path.to_owned());
                installed
                    .clusters
                    .extend(kubeconfig.clusters.drain(..).map(|x| Sourced::new(&path, x)));
                installed
                    .contexts
                    .extend(kubeconfig.contexts.drain(..).map(|x| Sourced::new(&path, x)));
                installed
                    .users
                    .extend(kubeconfig.users.drain(..).map(|x| Sourced::new(&path, x)));
            }
            Err(err) => {
                eprintln!("Error loading kubeconfig {}: {}", path.display(), err);
            }
        }
    }

    Ok(installed)
}

pub fn get_installed_contexts(settings: &Settings) -> Result<Installed> {
    let installed = load_kubeconfigs(settings.get_kube_configs_paths()?)?;
    if installed.contexts.is_empty() {
        bail!("Could not find any contexts in the Kubie kubeconfig directories!");
    }
    Ok(installed)
}

pub fn get_kubeconfigs_contexts(kubeconfigs: &Vec<String>) -> Result<Installed> {
    let installed = load_kubeconfigs(kubeconfigs)?;
    if installed.contexts.is_empty() {
        bail!("Could not find any contexts in the given set of files!");
    }
    Ok(installed)
}

pub fn get_kubeconfig_path() -> Result<PathBuf> {
    let path = env::var_os("KUBIE_KUBECONFIG").context("KUBIE_CONFIG not found")?;
    Ok(PathBuf::from(path))
}

pub fn get_current_config() -> Result<KubeConfig> {
    ioutil::read_yaml(get_kubeconfig_path()?)
}
