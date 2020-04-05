use std::env;

use anyhow::{anyhow, Context, Result};

use self::detect::{detect, ShellKind};
use crate::kubeconfig::KubeConfig;
use crate::session::Session;
use crate::settings::Settings;
use crate::tempfile::Tempfile;
use crate::vars;

mod bash;
mod detect;
mod zsh;

pub struct ShellInfo<'a> {
    settings: &'a Settings,
    temp_config_file: Tempfile,
    temp_session_file: Tempfile,
    next_depth: u32,
    path: String,
}

fn add_kubie_to_path_var() -> Result<String> {
    let current_exe_path = env::current_exe().context("Could not get current exe path")?;
    let current_exe_parent = current_exe_path
        .parent()
        .ok_or_else(|| anyhow!("Current exe path has no parent"))?;
    let kubie_dir = current_exe_parent
        .to_str()
        .ok_or_else(|| anyhow!("Current exe path contains non-unicode characters"))?
        .to_owned();

    let path_var = env::var("PATH").unwrap_or("".into());

    let mut dirs: Vec<&str> = path_var.split(":").collect();
    if !dirs.contains(&kubie_dir.as_str()) {
        dirs.insert(0, kubie_dir.as_str());
    }

    Ok(dirs.join(":"))
}

pub fn spawn_shell(settings: &Settings, config: KubeConfig, session: &Session) -> Result<()> {
    let kind = match &settings.shell {
        Some(shell) => ShellKind::from_str(&shell).ok_or_else(|| anyhow!("Invalid shell setting: {}", shell))?,
        None => detect()?,
    };

    let temp_config_file = Tempfile::new("/tmp", "kubie-config", ".yaml")?;
    config.write_to(&*temp_config_file)?;

    let temp_session_file = Tempfile::new("/tmp", "kubie-session", ".yaml")?;
    session.save(Some(temp_session_file.path()))?;

    let depth = vars::get_depth();
    let next_depth = depth + 1;

    let info = ShellInfo {
        settings,
        temp_config_file,
        temp_session_file,
        next_depth,
        path: add_kubie_to_path_var()?,
    };

    match kind {
        ShellKind::Bash => bash::spawn_shell(&info),
        ShellKind::Zsh => zsh::spawn_shell(&info),
        _ => bash::spawn_shell(&info),
    }
}
