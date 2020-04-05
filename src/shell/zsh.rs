use std::fs::File;
use std::io::Write;
use std::process::Command;

use anyhow::{Context, Result};
use tempfile::tempdir;

use super::ShellInfo;

pub fn spawn_shell(info: &ShellInfo) -> Result<()> {
    let dir = tempdir()?;
    {
        let zshrc_path = dir.path().join(".zshrc");
        let mut zshrc = File::create(zshrc_path).context("Could not open zshrc file")?;
        write!(
            zshrc,
            r#"
if [ -f "$HOME/.zshrc" ] ; then
    source "$HOME/.zshrc"
fi

function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

autoload -Uz add-zsh-hook
add-zsh-hook preexec __kubie_cmd_pre_exec__

setopt PROMPT_SUBST
KUBIE_PROMPT='[$(kubie info ctx)|$(kubie info ns)]'

if [ "$KUBIE_ZSH_USE_RPS1" = "1" ] ; then
    RPS1="$KUBIE_PROMPT $RPS1"
else
    PS1="$KUBIE_PROMPT $PS1"
fi

unset KUBIE_PROMPT
"#
        )?;
    }

    let mut child = Command::new("zsh")
        .env("ZDOTDIR", dir.path())
        .env("PATH", &info.path)
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", info.next_depth.to_string())
        .env("KUBIE_KUBECONFIG", info.temp_config_file.path())
        .env("KUBIE_SESSION", info.temp_session_file.path())
        .env("KUBIE_ZSH_USE_RPS1", if info.settings.zsh_use_rps1 { "1" } else { "0" })
        .spawn()?;
    child.wait()?;
    Ok(())
}
