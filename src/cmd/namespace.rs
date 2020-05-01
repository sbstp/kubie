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

pub fn namespace(settings: &Settings, namespace_name: Option<String>, recursive: bool) -> Result<()> {
    vars::ensure_kubie_active()?;

    let namespaces = kubectl::get_namespaces(None)?;

    let enter_namespace = |mut namespace_name: String| -> Result<()> {
        let mut session = Session::load().context("Could not load session file")?;

        if namespace_name == "-" {
            namespace_name = session
                .get_last_namespace()
                .context("There is not previous namespace to switch to")?
                .to_string();
        } else if !namespaces.contains(&namespace_name) {
            return Err(anyhow!("'{}' is not a valid namespace for the context", namespace_name));
        }

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
    };

    let namespace_name = match namespace_name {
        Some(namespace_name) => namespace_name,
        None => match select_or_list_namespace()? {
            SelectResult::Selected(x) => x,
            _ => return Ok(()),
        },
    };

    enter_namespace(namespace_name)
}
