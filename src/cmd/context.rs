use std::fs::File;

use anyhow::{anyhow, Context, Result};

use crate::cmd::{select_or_list, SelectResult};
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
    let mut session = Session::load()?;
    let kubeconfig = if context_name == "-" {
        let previous_ctx = session
            .get_last_context()
            .context("There is not previous context to switch to.")?;
        installed.make_kubeconfig_for_context(&previous_ctx.context, Some(&previous_ctx.namespace))?
    } else {
        installed.make_kubeconfig_for_context(&context_name, namespace_name)?
    };

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
    mut namespace_name: Option<String>,
    recursive: bool,
) -> Result<()> {
    let mut installed = kubeconfig::get_installed_contexts(settings)?;

    let context_name = match context_name {
        Some(context_name) => context_name,
        None => match select_or_list(&mut installed)? {
            SelectResult::Selected(x) => {
                namespace_name = None;
                x
            }
            _ => return Ok(()),
        },
    };

    enter_context(settings, installed, &context_name, namespace_name.as_deref(), recursive)
}
