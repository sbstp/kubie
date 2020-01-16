use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use structopt::StructOpt;

use tempfile::Tempfile;

mod kubeconfig;
mod kubectl;
mod tempfile;

struct Config {
    name: String,
    path: PathBuf,
}

fn get_configs() -> Result<Vec<Config>> {
    let mut config_dir = dirs::home_dir().ok_or(anyhow!("Could not get home directory"))?;
    config_dir.push(".kube");
    config_dir.push("kubie");

    let dir_iter = config_dir.read_dir().context(format!(
        "Could not list list files in {}",
        config_dir.display()
    ))?;

    let mut configs = vec![];

    for entry in dir_iter {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_file() {
            if let (Some(stem), Some(ext)) = (
                path.file_stem().and_then(|s| s.to_str()),
                path.extension().and_then(|s| s.to_str()),
            ) {
                match ext {
                    "yml" | "yaml" => {
                        configs.push(Config {
                            name: stem.to_string(),
                            path: path.clone(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    configs.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(configs)
}

fn get_depth() -> u32 {
    env::var("KUBIE_DEPTH")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

fn get_context_config(context_name: &str) -> Result<Config> {
    for c in get_configs()? {
        if c.name.to_lowercase() == context_name.to_lowercase() {
            return Ok(c);
        }
    }

    return Err(anyhow!("Context {} not found", context_name));
}

fn ensure_kubie_shell() -> Result<()> {
    let active = env::var("KUBIE_ACTIVE").unwrap_or("0".into());
    if active != "1" {
        return Err(anyhow!("Not in a kubie shell!"));
    }
    Ok(())
}

fn spawn_shell(
    context_config: &Config,
    namespace_name: Option<&str>,
    shell: &OsStr,
    depth: u32,
) -> Result<()> {
    let mut config = kubeconfig::load(&context_config.path)?;
    if let Some(ns) = namespace_name {
        config.contexts[0].context.namespace = ns.to_string();
    }
    let temp_config_file = Tempfile::new("/tmp", "kubie-config", ".yaml")?;
    config.write_to(&*temp_config_file)?;

    let mut temp_rc_file = Tempfile::new("/tmp", "kubie-bashrc-", ".bash")?;
    write!(
        temp_rc_file,
        r#"
if [ -f "$HOME/.bashrc" ] ; then
    source "$HOME/.bashrc"
fi

if [ -f "/etc/skel/.bashrc" ] ; then
    source "/etc/skel/.bashrc"
fi

function kubectx {{
    echo "kubectx disabled to prevent misuse."
}}

function kubens {{
    echo "kubens disabled to prevent misuse."
}}

export KUBECONFIG="{}"
export PATH="{}:$PATH"

PROMPT='\[[\033[0;32m\]$(kubie info ctx)\[\033[m\]|\[\033[0;31m\]$(kubie info ns)\[\033[m]\]'
export PS1="$PROMPT  ${{PS1}}"
unset PROMPT
"#,
        temp_config_file.path().display(),
        env::current_exe().unwrap().parent().unwrap().display(),
    )?;

    let mut child = Command::new(shell)
        .arg("--rcfile")
        .arg(temp_rc_file.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", format!("{}", depth + 1))
        .spawn()?;
    child.wait()?;

    println!("Kubie depth now {}", depth);

    Ok(())
}

#[derive(Debug, StructOpt)]
enum InfoItem {
    #[structopt(name = "ctx")]
    Context,
    #[structopt(name = "ns")]
    Namespace,
}

#[derive(Debug, StructOpt)]
enum Kubie {
    #[structopt(name = "ctx", about = "Spawn a new shell in the given context")]
    Context {
        #[structopt(
            short = "n",
            help = "Specify the namespace in which the shell is spawned"
        )]
        namespace_name: Option<String>,
        context_name: Option<String>,
    },
    #[structopt(
        name = "ns",
        about = "Spawn a new shell in the given namespace, using the current context"
    )]
    Namespace { namespace_name: Option<String> },
    #[structopt(name = "info", about = "View info about the environment")]
    Info(InfoItem),
}

fn main() -> Result<()> {
    let kubie = Kubie::from_args();

    let shell = env::var_os("SHELL").unwrap_or("/bin/bash".into());
    let depth = get_depth();

    match kubie {
        Kubie::Context {
            namespace_name,
            context_name,
        } => {
            if let Some(context_name) = context_name {
                let config = get_context_config(&context_name)?;
                spawn_shell(&config, namespace_name.as_deref(), &shell, depth)?;
            } else {
                for c in get_configs()? {
                    println!("{}", c.name);
                }
            }
        }
        Kubie::Namespace { namespace_name } => {
            if let Some(namespace_name) = namespace_name {
                ensure_kubie_shell()?;
                let mut config = kubeconfig::get_current_config()?;
                config.contexts[0].context.namespace = namespace_name;

                let config_file = File::create(kubeconfig::get_kubeconfig_path()?)?;
                config.write_to(config_file)?;
            } else {
                for ns in kubectl::get_namespaces()? {
                    println!("{}", ns);
                }
            }
        }
        Kubie::Info(item) => match item {
            InfoItem::Context => {
                ensure_kubie_shell()?;
                let conf = kubeconfig::get_current_config()?;
                println!("{}", conf.current_context.as_deref().unwrap_or(""));
            }
            InfoItem::Namespace => {
                ensure_kubie_shell()?;
                let conf = kubeconfig::get_current_config()?;
                println!("{}", conf.contexts[0].context.namespace);
            }
        },
    }

    Ok(())
}
