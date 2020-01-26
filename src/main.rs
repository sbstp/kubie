use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use anyhow::{anyhow, Result};
use structopt::StructOpt;
use tempfile::Tempfile;

use commands::{Kubie, KubieInfoKind};

mod commands;
mod kubeconfig;
mod kubectl;
mod tempfile;
mod vars;

/// Get the current depth of shells.
fn get_depth() -> u32 {
    env::var("KUBIE_DEPTH")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

/// Get the next depth if a context is created.
fn get_next_depth() -> String {
    format!("{}", get_depth() + 1)
}

/// Ensure that we're inside a kubie shell, returning an error if we aren't.
fn ensure_kubie_active() -> Result<()> {
    let active = env::var("KUBIE_ACTIVE").unwrap_or("0".into());
    if active != "1" {
        return Err(anyhow!("Not in a kubie shell!"));
    }
    Ok(())
}

fn spawn_shell(config: kubeconfig::KubeConfig, shell: &OsStr, depth: u32) -> Result<()> {
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

function k {{
    echo "k on disabled to prevent misuse."
}}

export KUBECONFIG="{}"
export PATH="{}"

PROMPT='{}'
export PS1="$PROMPT ${{PS1}}"
unset PROMPT
"#,
        temp_config_file.path().display(),
        vars::generate_path()?,
        vars::generate_ps1(depth + 1),
    )?;

    let mut child = Command::new(shell)
        .arg("--rcfile")
        .arg(temp_rc_file.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", get_next_depth())
        .spawn()?;
    child.wait()?;

    println!("Kubie depth is now {}", depth);

    Ok(())
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
            let mut installed = kubeconfig::get_installed_contexts()?;

            if let Some(context_name) = context_name {
                let kubeconfig = installed.make_kubeconfig_for_context(&context_name, namespace_name.as_deref())?;
                spawn_shell(kubeconfig, &shell, depth)?;
            } else {
                installed.contexts.sort_by(|a, b| a.name.cmp(&b.name));
                for c in installed.contexts {
                    println!("{}", c.name);
                }
            }
        }
        Kubie::Namespace { namespace_name } => {
            if let Some(namespace_name) = namespace_name {
                ensure_kubie_active()?;
                let namespaces = kubectl::get_namespaces()?;
                if !namespaces.contains(&namespace_name) {
                    return Err(anyhow!("{} is not a valid namespace for the context", namespace_name));
                }
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
        Kubie::Info(info) => match info.kind {
            KubieInfoKind::Context => {
                ensure_kubie_active()?;
                let conf = kubeconfig::get_current_config()?;
                println!("{}", conf.current_context.as_deref().unwrap_or(""));
            }
            KubieInfoKind::Namespace => {
                ensure_kubie_active()?;
                let conf = kubeconfig::get_current_config()?;
                println!("{}", conf.contexts[0].context.namespace);
            }
            KubieInfoKind::Depth => {
                ensure_kubie_active()?;
                println!("{}", get_depth());
            }
        },
        Kubie::Exec {
            context_name,
            namespace_name,
            args,
        } => {
            if args.len() == 0 {
                return Ok(());
            }

            let installed = kubeconfig::get_installed_contexts()?;
            let kubeconfig = installed.make_kubeconfig_for_context(&context_name, Some(&namespace_name))?;

            let temp_config_file = Tempfile::new("/tmp", "kubie-config", ".yaml")?;
            kubeconfig.write_to(&*temp_config_file)?;

            let mut proc = Command::new(&args[0])
                .args(&args[1..])
                .env("KUBECONFIG", temp_config_file.path())
                .env("KUBIE_ACTIVE", "1")
                .env("KUBIE_DEPTH", get_next_depth())
                .spawn()?;
            let status = proc.wait()?;
            std::process::exit(status.code().unwrap_or(0))
        }
    }

    Ok(())
}
