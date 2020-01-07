use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{anyhow, Context, Result};

struct Config {
    name: String,
    path: PathBuf,
}

fn get_configs() -> Result<Vec<Config>> {
    let mut config_dir = dirs::home_dir().ok_or(anyhow!("Could not get home directory"))?;
    config_dir.push(".kube");
    config_dir.push("kubie");

    let dir_iter = config_dir.read_dir().context(format!(
        "Could not list list files in {}",
        config_dir.display()
    ))?;

    let mut configs = vec![];

    for entry in dir_iter {
        let entry = entry?;
        let path = entry.path();

        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            configs.push(Config {
                name: stem.to_string(),
                path: path,
            })
        }
    }

    Ok(configs)
}

fn get_depth() -> u32 {
    env::var("KUBIE_DEPTH")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

fn get_context_config() -> Result<Config> {
    let context_name = env::args().nth(1).ok_or(anyhow!("Missing context name"))?;

    for c in get_configs()? {
        if c.name.to_lowercase() == context_name.to_lowercase() {
            return Ok(c);
        }
    }

    return Err(anyhow!("Context {} not found", context_name));
}

fn main() -> Result<()> {
    let shell = env::var_os("SHELL").unwrap_or("/bin/bash".into());
    let path = env::var_os("PATH").unwrap();
    let depth = get_depth();
    let context_config = get_context_config()?;

    let mut rcfile = File::create("/tmp/kubie.sh")?;
    write!(
        rcfile,
        r#"\
if [ -f "$HOME/.bashrc" ] ; then
    source "$HOME/.bashrc"
fi

if [ -f "/etc/skel/.bashrc" ] ; then
    source "/etc/skel/.bashrc"
fi

KUBECONFIG="{}"
PS1='[\e[0;32m$(kubectl config current-context)\e[m|\e[0;31m$(kubectl config view | grep namespace | awk '{{print $2}}')\e[m]'
PS1+=" ${{PS1}}"
"#,
        context_config.path.display()
    )?;

    let mut new_path = OsString::new();
    new_path.push(env::current_exe().unwrap().parent().unwrap());
    new_path.push(":");
    new_path.push(path);

    let mut child = Command::new(shell)
        .arg("--rcfile")
        .arg("/tmp/kubie.sh")
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", format!("{}", depth + 1))
        .env("PATH", new_path)
        // create bashrc file to overwrite PS1
        // use --rcfile when spawning shell
        .spawn()?;
    child.wait()?;

    println!("Kubie depth now {}", depth);

    Ok(())
}
