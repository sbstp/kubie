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
autoload -Uz add-zsh-hook
function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

autoload -Uz add-zsh-hook
add-zsh-hook preexec __kubie_cmd_pre_exec__

function kubie() {{
    echo "hello"
}}

setopt PROMPT_SUBST
RPS1='[$(kubie info ctx)|$(kubie info ns)]'
#RPS1="$KUBIE_PROMPT
"#
        )?;
    }

    let mut child = Command::new("zsh")
        .env("ZDOTDIR", dir.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", info.next_depth.to_string())
        .env("KUBIE_KUBECONFIG", info.temp_config_file.path())
        .env("KUBIE_SESSION", info.temp_session_file.path())
        .spawn()?;
    child.wait()?;
    Ok(())
}
