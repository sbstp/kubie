use std::io::Cursor;

use anyhow::{bail, Result};
use skim::prelude::SkimItemReader;
use skim::{Skim, SkimOptions};

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
    Selected(String),
}

pub fn select_context(skim_options: &SkimOptions, installed: &mut Installed) -> Result<SelectResult> {
    installed.contexts.sort_by(|a, b| a.item.name.cmp(&b.item.name));
    let mut context_names: Vec<_> = installed.contexts.iter().map(|c| c.item.name.clone()).collect();
    context_names.reverse();

    if context_names.is_empty() {
        bail!("No contexts found");
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(context_names.join("\n")));
    let selected_items = Skim::run_with(skim_options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    if selected_items.is_empty() {
        return Ok(SelectResult::Cancelled);
    }
    Ok(SelectResult::Selected(selected_items[0].output().to_string()))
}

pub fn select_namespace(skim_options: &SkimOptions) -> Result<SelectResult> {
    let mut namespaces = kubectl::get_namespaces(None)?;
    namespaces.reverse();

    if namespaces.is_empty() {
        bail!("No namespaces found");
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(namespaces.join("\n")));
    let selected_items = Skim::run_with(skim_options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    if selected_items.is_empty() {
        return Ok(SelectResult::Cancelled);
    }
    Ok(SelectResult::Selected(selected_items[0].output().to_string()))
}
