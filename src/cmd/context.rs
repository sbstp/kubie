use anyhow::{Context, Result};
use skim::SkimOptions;

use crate::cmd::{select_or_list_context, SelectResult};
use crate::kubeconfig::{self, Installed};
use crate::kubectl;
use crate::session::Session;
use crate::settings::Settings;
use crate::shell::spawn_shell;
use crate::state::State;
use crate::vars;

fn enter_context(
    settings: &Settings,
    installed: Installed,
    context_name: &str,
    namespace_name: Option<&str>,
    recursive: bool,
) -> Result<()> {
    let state = State::load()?;
    let mut session = Session::load()?;

    let namespace_name =
        namespace_name.or_else(|| state.namespace_history.get(context_name).and_then(|s| s.as_deref()));

    let kubeconfig = if context_name == "-" {
        let previous_ctx = session
            .get_last_context()
            .context("There is no previous context to switch to.")?;
        installed.make_kubeconfig_for_context(&previous_ctx.context, previous_ctx.namespace.as_deref())?
    } else {
        installed.make_kubeconfig_for_context(context_name, namespace_name)?
    };

    session.add_history_entry(
        &kubeconfig.contexts[0].name,
        kubeconfig.contexts[0].context.namespace.as_deref(),
    );

    if settings.behavior.validate_namespaces {
        if let Some(namespace_name) = namespace_name {
            let namespaces = kubectl::get_namespaces(Some(&kubeconfig))?;
            if !namespaces.iter().any(|x| x == namespace_name) {
                eprintln!("Warning: namespace {} does not exist.", namespace_name);
            }
        }
    }

    if vars::is_kubie_active() && !recursive {
        let path = kubeconfig::get_kubeconfig_path()?;
        kubeconfig.write_to_file(path.as_path())?;
        session.save(None)?;
    } else {
        spawn_shell(settings, kubeconfig, &session)?;
    }

    Ok(())
}

pub fn context(
    settings: &Settings,
    skim_options: &SkimOptions,
    context_name: Option<String>,
    mut namespace_name: Option<String>,
    kubeconfigs: Vec<String>,
    recursive: bool,
) -> Result<()> {
    let mut installed = if kubeconfigs.is_empty() {
        kubeconfig::get_installed_contexts(settings)?
    } else {
        kubeconfig::get_kubeconfigs_contexts(&kubeconfigs)?
    };

    let context_name = match context_name {
        Some(context_name) => context_name,
        None => match select_or_list_context(skim_options, &mut installed)? {
            SelectResult::Selected(x) => {
                namespace_name = None;
                x
            }
            _ => return Ok(()),
        },
    };

    enter_context(settings, installed, &context_name, namespace_name.as_deref(), recursive)
}
