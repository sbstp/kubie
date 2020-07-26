use std::fmt::Display;

use crate::{kubeconfig, kubectl, settings::Settings};

const SUB_COMMANDS: &'static [&'static str] = &[
    "ctx",
    "ns",
    "info",
    "exec",
    "lint",
    "edit",
    "edit-config",
    "update",
    "delete",
];

fn print_options<I, T, F>(options: I, filter: Option<F>)
where
    I: IntoIterator<Item = T>,
    T: Display,
    F: Display,
{
    if let Some(filter) = filter {
        let filter = filter.to_string();
        for x in options
            .into_iter()
            .map(|x| x.to_string())
            .filter(|x| x.starts_with(&filter))
        {
            println!("{}", x);
        }
    } else {
        for x in options {
            println!("{}", x);
        }
    }
}

pub fn get_completions(settings: &Settings, position: usize, line: String) -> anyhow::Result<()> {
    let installed = kubeconfig::get_installed_contexts(settings)?;
    let words: Vec<_> = line.trim().split_whitespace().collect();

    match position {
        1 => {
            print_options(SUB_COMMANDS.iter(), words.get(position));
        }
        2 => match words[1] {
            "ctx" | "exec" | "edit" | "delete" => {
                print_options(installed.contexts.iter().map(|x| &x.item.name), words.get(position));
            }
            "ns" => {
                print_options(kubectl::get_namespaces(None)?, words.get(position));
            }
            "info" => {
                print_options(&["ctx", "ns", "depth"], words.get(position));
            }
            _ => {}
        },
        3 => match (words[1], words[2]) {
            ("ctx", _) => {
                print_options(&["--namespace", "--kubeconfig", "--recursive"], words.get(position));
            }
            ("ns", _) => {
                print_options(&["--recursive"], words.get(position));
            }
            ("exec", context_name) => {
                let kubeconfig = installed.make_kubeconfig_for_context(context_name, None)?;
                let namespaces = kubectl::get_namespaces(Some(&kubeconfig))?;
                print_options(namespaces, words.get(position));
            }
            _ => {}
        },
        _ => match (words[1], words[2], words.get(position - 1).map(|&s| s)) {
            ("ctx", context_name, Some("--namespace")) => {
                let kubeconfig = installed.make_kubeconfig_for_context(context_name, None)?;
                let namespaces = kubectl::get_namespaces(Some(&kubeconfig))?;
                print_options(namespaces, words.get(position));
            }
            ("exec", _, _) if position >= 4 => {
                print_options(&["--exit-early", "--"], words.get(position));
            }
            _ => {}
        },
    }

    Ok(())
}
