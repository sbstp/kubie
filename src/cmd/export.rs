use anyhow::{anyhow, Result};

use crate::kubeconfig;
use crate::settings::Settings;

pub fn export(settings: &Settings, context_name: String, namespace_name: String) -> Result<()> {
    let installed = kubeconfig::get_installed_contexts(settings)?;
    let matching = installed.get_contexts_matching(&context_name, settings.behavior.allow_multiple_context_patterns);

    if matching.is_empty() {
        return Err(anyhow!("No context matching {}", context_name));
    }

    for context_src in matching {
        let kubeconfig = installed.make_kubeconfig_for_context(&context_src.item.name, Some(&namespace_name))?;
        let temp_config_file = tempfile::Builder::new()
            .prefix("kubie-config")
            .suffix(".yaml")
            .tempfile()?;
        kubeconfig.write_to_file(temp_config_file.path())?;
        let (_, path) = temp_config_file.keep()?;
        println!("{}", path.display());
    }

    std::process::exit(0);
}
