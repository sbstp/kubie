use std::process::Command;
use std::thread;

use anyhow::Result;
use signal_hook::iterator::Signals;

use crate::kubeconfig::{self, KubeConfig};
use crate::settings::Settings;
use crate::vars;

fn run_in_context(kubeconfig: &KubeConfig, args: &[String]) -> anyhow::Result<i32> {
    let temp_config_file = tempfile::Builder::new()
        .prefix("kubie-config")
        .suffix(".yaml")
        .tempfile()?;
    kubeconfig.write_to(&temp_config_file)?;

    let depth = vars::get_depth();
    let next_depth = depth + 1;

    let signals = Signals::new(&[
        signal_hook::SIGHUP,
        signal_hook::SIGTERM,
        signal_hook::SIGINT,
        signal_hook::SIGQUIT,
        signal_hook::SIGWINCH,
        signal_hook::SIGUSR1,
        signal_hook::SIGUSR2,
    ])
    .expect("could not install signal handler");

    let mut child = Command::new(&args[0])
        .args(&args[1..])
        .env("KUBECONFIG", temp_config_file.path())
        .env("KUBIE_KUBECONFIG", temp_config_file.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", next_depth.to_string())
        .spawn()?;

    let child_pid = child.id();

    thread::spawn(move || {
        for sig in signals.forever() {
            unsafe {
                libc::kill(child_pid as libc::pid_t, sig as libc::c_int);
            }
        }
    });

    let status = child.wait()?;

    Ok(status.code().unwrap_or(0))
}

pub fn exec(
    settings: &Settings,
    context_name: String,
    namespace_name: String,
    exit_early: bool,
    args: Vec<String>,
) -> Result<()> {
    if args.len() == 0 {
        return Ok(());
    }

    let installed = kubeconfig::get_installed_contexts(settings)?;

    for context_src in installed.get_contexts_matching(&context_name) {
        let kubeconfig = installed.make_kubeconfig_for_context(&context_src.item.name, Some(&namespace_name))?;
        let return_code = run_in_context(&kubeconfig, &args)?;

        if return_code != 0 && exit_early {
            std::process::exit(return_code);
        }
    }

    std::process::exit(0);
}
