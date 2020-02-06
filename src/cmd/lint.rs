use std::collections::HashSet;
use std::path::Path;

use anyhow::Result;

use crate::kubeconfig::{self, Installed};
use crate::settings::Settings;

fn lint_clusters(installed: &Installed) {
    let mut set: HashSet<(&str, &Path)> = HashSet::new();

    for cluster_src in &installed.clusters {
        let named = &cluster_src.item;

        if installed
            .find_contexts_by_cluster(&named.name, &cluster_src.source)
            .is_empty()
        {
            println!(
                "Cluster '{}' has no context referencing it in file {}",
                named.name,
                cluster_src.source.display(),
            );
        }
        if set.contains(&(&named.name, &cluster_src.source)) {
            println!(
                "A cluster named '{}' appears more than once in file {}",
                named.name,
                cluster_src.source.display(),
            );
        } else {
            set.insert((&named.name, &cluster_src.source));
        }
    }
}

fn lint_users(installed: &Installed) {
    let mut set: HashSet<(&str, &Path)> = HashSet::new();

    for user_src in &installed.users {
        let named = &user_src.item;

        if installed
            .find_contexts_by_user(&named.name, &user_src.source)
            .is_empty()
        {
            println!(
                "User '{}' has no context referencing it in file {}",
                named.name,
                user_src.source.display(),
            );
        }
        if set.contains(&(&named.name, &user_src.source)) {
            println!(
                "A user named '{}' appears more than once in file {}",
                named.name,
                user_src.source.display(),
            );
        } else {
            set.insert((&named.name, &user_src.source));
        }
    }
}

fn lint_contexts(installed: &Installed) {
    let mut set = HashSet::new();

    for context_src in &installed.contexts {
        let named = &context_src.item;

        if installed
            .find_cluster_by_name(&named.context.cluster, &context_src.source)
            .is_none()
        {
            println!(
                "Context '{}' references unknown cluster '{}' in file {}",
                named.name,
                named.context.cluster,
                context_src.source.display(),
            );
        }
        if installed
            .find_user_by_name(&named.context.user, &context_src.source)
            .is_none()
        {
            println!(
                "Context '{}' references unknown users '{}' in file {}",
                named.name,
                named.context.user,
                context_src.source.display(),
            );
        }
        if set.contains(&named.name) {
            println!(
                "A context name '{}' appears more than once in file {}",
                named.name,
                context_src.source.display()
            );
        } else {
            set.insert(&named.name);
        }
    }
}

pub fn lint(settings: &Settings) -> Result<()> {
    let installed = kubeconfig::get_installed_contexts(settings)?;
    lint_clusters(&installed);
    lint_users(&installed);
    lint_contexts(&installed);
    Ok(())
}
