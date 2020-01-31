use std::io::Write;
use std::process::Command;

use anyhow::{anyhow, Result};

use crate::fzf;
use crate::kubeconfig::{self, Installed};
use crate::kubectl;
use crate::tempfile::Tempfile;
use crate::vars;

fn spawn_shell(config: kubeconfig::KubeConfig, depth: u32) -> Result<()> {
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

    let mut child = Command::new("bash")
        .arg("--rcfile")
        .arg(temp_rc_file.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", vars::get_next_depth())
        .spawn()?;
    child.wait()?;

    println!("Kubie depth is now {}", depth);

    Ok(())
}

fn enter_context(installed: Installed, context_name: &str, namespace_name: Option<&str>) -> Result<()> {
    let depth = vars::get_depth();
    let kubeconfig = installed.make_kubeconfig_for_context(&context_name, namespace_name)?;

    if let Some(namespace_name) = namespace_name {
        let namespaces = kubectl::get_namespaces(Some(&kubeconfig))?;
        if !namespaces.contains(&namespace_name.to_string()) {
            return Err(anyhow!("'{}' is not a valid namespace for the context", namespace_name));
        }
    }

    spawn_shell(kubeconfig, depth)?;
    Ok(())
}

pub fn context(context_name: Option<String>, namespace_name: Option<String>) -> Result<()> {
    let mut installed = kubeconfig::get_installed_contexts()?;

    if let Some(context_name) = context_name {
        enter_context(installed, &context_name, namespace_name.as_deref())?;
    } else {
        installed.contexts.sort_by(|a, b| a.name.cmp(&b.name));

        // We only select the context with fzf if stdout is a terminal and if
        // fzf is present on the machine.
        if atty::is(atty::Stream::Stdout) && fzf::is_available() {
            match crate::fzf::select(installed.contexts.iter().map(|c| &c.name))? {
                Some(context_name) => {
                    enter_context(installed, &context_name, None)?;
                }
                None => {
                    println!("Selection cancelled.");
                }
            }
        } else {
            installed.contexts.sort_by(|a, b| a.name.cmp(&b.name));
            for c in installed.contexts {
                println!("{}", c.name);
            }
        }
    }

    Ok(())
}
