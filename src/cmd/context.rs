use std::fs::File;

use anyhow::{anyhow, Context, Result};

use crate::fzf;
use crate::kubeconfig::{self, Installed};
use crate::kubectl;
use crate::session::Session;
use crate::settings::Settings;
use crate::shell::spawn_shell;
use crate::vars;

fn enter_context(
    settings: &Settings,
    installed: Installed,
    context_name: &str,
    namespace_name: Option<&str>,
    recursive: bool,
) -> Result<()> {
    let kubeconfig = installed.make_kubeconfig_for_context(&context_name, namespace_name)?;

    let mut session = Session::load()?;
    session.add_history_entry(&kubeconfig.contexts[0].name, &kubeconfig.contexts[0].context.namespace);

    if let Some(namespace_name) = namespace_name {
        let namespaces = kubectl::get_namespaces(Some(&kubeconfig))?;
        if !namespaces.contains(&namespace_name.to_string()) {
            return Err(anyhow!("'{}' is not a valid namespace for the context", namespace_name));
        }
    }

    if vars::is_kubie_active() && !recursive {
        let path = kubeconfig::get_kubeconfig_path()?;
        let file = File::create(&path).context("could not write in temporary KUBECONFIG file")?;
        kubeconfig.write_to(file)?;
        session.save(None)?;
    } else {
        spawn_shell(settings, kubeconfig, &session)?;
    }

    Ok(())
}

pub fn context(
    settings: &Settings,
    context_name: Option<String>,
    namespace_name: Option<String>,
    recursive: bool,
) -> Result<()> {
    let mut installed = kubeconfig::get_installed_contexts(settings)?;

    if let Some(context_name) = context_name {
        enter_context(settings, installed, &context_name, namespace_name.as_deref(), recursive)?;
    } else {
        installed.contexts.sort_by(|a, b| a.item.name.cmp(&b.item.name));

        // We only select the context with fzf if stdout is a terminal and if
        // fzf is present on the machine.
        if atty::is(atty::Stream::Stdout) && fzf::is_available() {
            match fzf::select(installed.contexts.iter().map(|c| &c.item.name))? {
                Some(context_name) => {
                    enter_context(settings, installed, &context_name, None, recursive)?;
                }
                None => {
                    println!("Selection cancelled.");
                }
            }
        } else {
            installed.contexts.sort_by(|a, b| a.item.name.cmp(&b.item.name));
            for c in installed.contexts {
                println!("{}", c.item.name);
            }
        }
    }

    Ok(())
}
