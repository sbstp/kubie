use std::collections::HashMap;
use std::ffi::OsString;
use std::io::BufWriter;
use std::process::Command;

use anyhow::{anyhow, Result};

use self::detect::{detect, ShellKind};
use crate::kubeconfig::KubeConfig;
use crate::session::Session;
use crate::settings::Settings;
use crate::state;
use crate::vars;

mod bash;
mod detect;
mod prompt;
mod zsh;

pub struct EnvVars<'n> {
    vars: HashMap<&'n str, OsString>,
}

impl<'n> EnvVars<'n> {
    pub fn new() -> EnvVars<'n> {
        EnvVars { vars: HashMap::new() }
    }

    pub fn insert(&mut self, name: &'n str, value: impl Into<OsString>) {
        self.vars.insert(name, value.into());
    }

    pub fn apply(&self, cmd: &mut Command) {
        for (name, value) in &self.vars {
            cmd.env(name, value);
        }
    }
}

pub struct ShellSpawnInfo<'s, 'n> {
    settings: &'s Settings,
    env_vars: EnvVars<'n>,
    prompt: String,
}

pub fn spawn_shell(settings: &Settings, config: KubeConfig, session: &Session) -> Result<()> {
    let kind = match &settings.shell {
        Some(shell) => ShellKind::from_str(&shell).ok_or_else(|| anyhow!("Invalid shell setting: {}", shell))?,
        None => detect()?,
    };

    let temp_config_file = tempfile::Builder::new()
        .prefix("kubie-config")
        .suffix(".yaml")
        .tempfile()?;
    let temp_config_file_buf = BufWriter::new(temp_config_file.as_file());
    config.write_to(temp_config_file_buf)?;

    let temp_session_file = tempfile::Builder::new()
        .prefix("kubie-session")
        .suffix(".json")
        .tempfile()?;
    session.save(Some(temp_session_file.path()))?;

    let depth = vars::get_depth();
    let next_depth = depth + 1;

    let mut env_vars = EnvVars::new();

    // Pre-insert the KUBECONFIG variable into the shell.
    // This will make sure any shell plugins/add-ons which require this env variable
    // will have it available at the beginninng of the .rc file
    env_vars.insert("KUBECONFIG", temp_config_file.path());
    env_vars.insert("KUBIE_ACTIVE", "1");
    env_vars.insert("KUBIE_DEPTH", next_depth.to_string());
    env_vars.insert("KUBIE_KUBECONFIG", temp_config_file.path());
    env_vars.insert("KUBIE_SESSION", temp_session_file.path());
    env_vars.insert("KUBIE_STATE", state::path());

    env_vars.insert("KUBIE_PROMPT_DISABLE", if settings.prompt.disable { "1" } else { "0" });
    env_vars.insert(
        "KUBIE_ZSH_USE_RPS1",
        if settings.prompt.zsh_use_rps1 { "1" } else { "0" },
    );

    match kind {
        ShellKind::Bash => {
            env_vars.insert("KUBIE_SHELL", "bash");
        }
        ShellKind::Fish => {
            env_vars.insert("KUBIE_SHELL", "fish");
        }
        ShellKind::Zsh => {
            env_vars.insert("KUBIE_SHELL", "zsh");
        }
    }

    let info = ShellSpawnInfo {
        settings,
        env_vars,
        prompt: prompt::generate_ps1(settings, next_depth, kind),
    };

    match kind {
        ShellKind::Bash => bash::spawn_shell(&info),
        ShellKind::Zsh => zsh::spawn_shell(&info),
        _ => bash::spawn_shell(&info),
    }
}
