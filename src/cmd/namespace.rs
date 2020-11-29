use std::fs::File;

use anyhow::{anyhow, Context, Result};

use crate::cmd::{select_or_list_namespace, SelectResult};
use crate::kubeconfig;
use crate::kubectl;
use crate::session::Session;
use crate::settings::Settings;
use crate::shell::spawn_shell;
use crate::state::State;
use crate::vars;

pub fn namespace(settings: &Settings, namespace_name: Option<String>, recursive: bool, unset: bool) -> Result<()> {
    vars::ensure_kubie_active()?;

    let mut session = Session::load().context("Could not load session file")?;

    if namespace_name.is_none() && unset {
        return enter_namespace(settings, &mut session, recursive, None);
    }

    let namespaces = kubectl::get_namespaces(None)?;

    let namespace_name = match namespace_name {
        Some(s) if s == "-" => Some(
            session
                .get_last_namespace()
                .context("There is not previous namespace to switch to")?
                .to_string(),
        ),
        Some(s) if !namespaces.contains(&s) => return Err(anyhow!("'{}' is not a valid namespace for the context", s)),
        None => match select_or_list_namespace()? {
            SelectResult::Selected(s) => Some(s),
            _ => return Ok(()),
        },
        Some(_) => namespace_name,
    };

    enter_namespace(settings, &mut session, recursive, namespace_name)
}

fn enter_namespace(
    settings: &Settings,
    session: &mut Session,
    recursive: bool,
    namespace_name: Option<String>,
) -> Result<()> {
    let mut config = kubeconfig::get_current_config()?;
    config.contexts[0].context.namespace = namespace_name.clone();

    let context_name = &config.contexts[0].name;

    // Update the state, set the last namespace used for the context.
    let mut state = State::load().context("Could not load state file.")?;
    state
        .namespace_history
        .insert(context_name.into(), namespace_name.clone());
    state.save()?;

    // Update the history, add the context and namespace to it.
    session.add_history_entry(context_name, namespace_name);

    if recursive {
        spawn_shell(settings, config, &session)?;
    } else {
        let config_file = File::create(kubeconfig::get_kubeconfig_path()?)?;
        config.write_to(config_file)?;
        session.save(None)?;
    }

    Ok(())
}
