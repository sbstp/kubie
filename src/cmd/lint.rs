use std::collections::HashSet;

use anyhow::Result;

use crate::kubeconfig::{self, Installed};

fn lint_clusters(installed: &Installed) {
    let mut set = HashSet::new();

    for cluster in &installed.clusters {
        if installed.find_contexts_by_cluster(&cluster.name).is_empty() {
            println!("Cluster '{}' has no context referencing it.", cluster.name);
        }
        if set.contains(&cluster.name) {
            println!(
                "A cluster named '{}' appears more than once in the configs.",
                cluster.name
            );
        } else {
            set.insert(&cluster.name);
        }
    }
}

fn lint_users(installed: &Installed) {
    let mut set = HashSet::new();

    for user in &installed.users {
        if installed.find_contexts_by_user(&user.name).is_empty() {
            println!("User '{}' has no context referencing it.", user.name);
        }
        if set.contains(&user.name) {
            println!("A user named '{}' appears more than once in the configs.", user.name);
        } else {
            set.insert(&user.name);
        }
    }
}

fn lint_contexts(installed: &Installed) {
    let mut set = HashSet::new();

    for context in &installed.contexts {
        if installed.find_cluster_by_name(&context.context.cluster).is_none() {
            println!(
                "Context '{}' references unknown cluster '{}'.",
                context.name, context.context.cluster
            );
        }
        if installed.find_user_by_name(&context.context.user).is_none() {
            println!(
                "Context '{}' references unknown users '{}'.",
                context.name, context.context.user
            );
        }
        if set.contains(&context.name) {
            println!(
                "A context name '{}' appears more than once in the configs.",
                context.name
            );
        } else {
            set.insert(&context.name);
        }
    }
}

pub fn lint() -> Result<()> {
    let installed = kubeconfig::get_installed_contexts()?;
    lint_clusters(&installed);
    lint_users(&installed);
    lint_contexts(&installed);
    Ok(())
}
