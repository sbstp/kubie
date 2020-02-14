use std::env;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, bail, Result};
use which::which;

use crate::fzf;
use crate::kubeconfig;
use crate::settings::Settings;

fn get_editor() -> Result<PathBuf> {
    env::var("EDITOR")
        .ok()
        .and_then(|editor| which(editor).ok())
        .or_else(|| {
            for editor in &["vim", "emacs", "vi", "nano"] {
                if let Ok(path) = which(editor) {
                    return Some(path);
                }
            }
            None
        })
        .ok_or_else(|| anyhow!("Could not find any editor to use"))
}

pub fn edit_context(settings: &Settings, context_name: Option<String>) -> Result<()> {
    let mut installed = kubeconfig::get_installed_contexts(settings)?;
    installed.contexts.sort_by(|a, b| a.item.name.cmp(&b.item.name));

    let context_name = match context_name {
        Some(context_name) => context_name,
        None => match fzf::select(installed.contexts.iter().map(|s| &s.item.name))? {
            Some(context_name) => context_name,
            None => bail!("No context selected"),
        },
    };

    let context_src = installed
        .find_context_by_name(&context_name)
        .ok_or_else(|| anyhow!("Could not find context {}", context_name))?;

    let editor = get_editor()?;

    let mut job = Command::new(editor).arg(context_src.source.as_ref()).spawn()?;
    job.wait()?;

    Ok(())
}

pub fn edit_config() -> Result<()> {
    let editor = get_editor()?;
    let settings_path = Settings::path();

    let mut job = Command::new(editor).arg(settings_path).spawn()?;
    job.wait()?;

    Ok(())
}
