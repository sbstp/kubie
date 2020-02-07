use std::process::Command;

use anyhow::Result;

use crate::kubeconfig::{self, KubeConfig};
use crate::settings::Settings;
use crate::tempfile::Tempfile;
use crate::vars;

fn run_in_context(kubeconfig: &KubeConfig, args: &[String]) -> anyhow::Result<i32> {
    let temp_config_file = Tempfile::new("/tmp", "kubie-config", ".yaml")?;
    kubeconfig.write_to(&*temp_config_file)?;

    let depth = vars::get_depth();
    let next_depth = depth + 1;

    let mut proc = Command::new(&args[0])
        .args(&args[1..])
        .env("KUBECONFIG", temp_config_file.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", next_depth.to_string())
        .spawn()?;
    let status = proc.wait()?;

    Ok(status.code().unwrap_or(0))
}

pub fn exec(settings: &Settings, context_name: String, namespace_name: String, args: Vec<String>) -> Result<()> {
    if args.len() == 0 {
        return Ok(());
    }

    let installed = kubeconfig::get_installed_contexts(settings)?;

    for context_src in installed.get_contexts_matching(&context_name) {
        let kubeconfig = installed.make_kubeconfig_for_context(&context_src.item.name, Some(&namespace_name))?;
        let return_code = run_in_context(&kubeconfig, &args)?;

        if return_code != 0 {
            std::process::exit(return_code);
        }
    }

    std::process::exit(0);
}
