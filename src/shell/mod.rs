mod bash;
mod detect;
mod zsh;

use anyhow::{anyhow, Result};

use self::detect::{detect, ShellKind};
use crate::kubeconfig::KubeConfig;
use crate::session::Session;
use crate::settings::Settings;
use crate::tempfile::Tempfile;
use crate::vars;

pub struct ShellInfo<'a> {
    settings: &'a Settings,
    session: &'a Session,
    config: KubeConfig,
    temp_config_file: Tempfile,
    temp_session_file: Tempfile,
    depth: u32,
    next_depth: u32,
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
        session,
        config,
        temp_config_file,
        temp_session_file,
        depth,
        next_depth,
    };

    match kind {
        ShellKind::Bash => bash::spawn_shell(&info),
        ShellKind::Zsh => zsh::spawn_shell(&info),
        _ => bash::spawn_shell(&info),
    }
}
