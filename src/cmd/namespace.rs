use std::fs::File;

use anyhow::{anyhow, Result};

use crate::kubeconfig;
use crate::kubectl;
use crate::vars;

pub fn namespace(namespace_name: Option<String>) -> Result<()> {
    if let Some(namespace_name) = namespace_name {
        vars::ensure_kubie_active()?;

        let namespaces = kubectl::get_namespaces(None)?;
        if !namespaces.contains(&namespace_name) {
            return Err(anyhow!("'{}' is not a valid namespace for the context", namespace_name));
        }

        let mut config = kubeconfig::get_current_config()?;
        config.contexts[0].context.namespace = namespace_name;

        let config_file = File::create(kubeconfig::get_kubeconfig_path()?)?;
        config.write_to(config_file)?;
    } else {
        for ns in kubectl::get_namespaces(None)? {
            println!("{}", ns);
        }
    }

    Ok(())
}
