use anyhow::Result;

use crate::fzf;
use crate::kubeconfig::Installed;
use crate::kubectl;

pub mod context;
pub mod delete;
pub mod edit;
pub mod exec;
pub mod info;
pub mod lint;
pub mod meta;
pub mod namespace;
pub mod update;

pub enum SelectResult {
    Cancelled,
    Listed,
    Selected(String),
}

pub fn select_or_list_context(installed: &mut Installed) -> Result<SelectResult> {
    installed.contexts.sort_by(|a, b| a.item.name.cmp(&b.item.name));

    // We only select the context with fzf if stdout is a terminal and if
    // fzf is present on the machine.
    Ok(if atty::is(atty::Stream::Stdout) && fzf::is_available() {
        match fzf::select(installed.contexts.iter().map(|c| &c.item.name))? {
            Some(context_name) => SelectResult::Selected(context_name),
            None => {
                println!("Selection cancelled.");
                SelectResult::Cancelled
            }
        }
    } else {
        for c in &installed.contexts {
            println!("{}", c.item.name);
        }
        SelectResult::Listed
    })
}

pub fn select_or_list_namespace() -> Result<SelectResult> {
    let mut namespaces = kubectl::get_namespaces(None)?;
    namespaces.sort();

    // We only select the namespace with fzf if stdout is a terminal and if
    // fzf is present on the machine.
    Ok(if atty::is(atty::Stream::Stdout) && fzf::is_available() {
        match fzf::select(namespaces.iter())? {
            Some(namespace_name) => SelectResult::Selected(namespace_name),
            None => {
                println!("Selection cancelled.");
                SelectResult::Cancelled
            }
        }
    } else {
        for ns in namespaces {
            println!("{}", ns);
        }
        SelectResult::Listed
    })
}
