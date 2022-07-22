use std::process::Command;
use std::thread;

use anyhow::{anyhow, Result};
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

use crate::kubeconfig::{self, KubeConfig};
use crate::settings::{ContextHeaderBehavior, Settings};
use crate::vars;

fn run_in_context(kubeconfig: &KubeConfig, args: &[String]) -> anyhow::Result<i32> {
    let temp_config_file = tempfile::Builder::new()
        .prefix("kubie-config")
        .suffix(".yaml")
        .tempfile()?;
    kubeconfig.write_to(&temp_config_file)?;

    let depth = vars::get_depth();
    let next_depth = depth + 1;

    let mut signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT, SIGWINCH, SIGUSR1, SIGUSR2])
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
    context_headers_flag: Option<ContextHeaderBehavior>,
    args: Vec<String>,
) -> Result<()> {
    if args.len() == 0 {
        return Ok(());
    }

    let installed = kubeconfig::get_installed_contexts(settings)?;
    let matching = installed.get_contexts_matching(&context_name);

    if matching.len() == 0 {
        return Err(anyhow!("No context matching {}", context_name));
    }

    let print_context = context_headers_flag
        .as_ref()
        .unwrap_or(&settings.behavior.print_context_in_exec)
        .should_print_headers();

    for context_src in matching {
        if print_context {
            println!("CONTEXT => {}", context_src.item.name);
        }
        let kubeconfig = installed.make_kubeconfig_for_context(&context_src.item.name, Some(&namespace_name))?;
        let return_code = run_in_context(&kubeconfig, &args)?;
        if print_context {
            println!("{}", "-".repeat(20));
        }

        if return_code != 0 && exit_early {
            std::process::exit(return_code);
        }
    }

    std::process::exit(0);
}
