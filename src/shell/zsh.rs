use std::fs::File;
use std::io::Write;
use std::process::Command;

use anyhow::{Context, Result};
use tempfile::tempdir;

use super::ShellSpawnInfo;

pub fn spawn_shell(info: &ShellSpawnInfo) -> Result<()> {
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
KUBIE_PROMPT=$'{}'

if [ "$KUBIE_ZSH_USE_RPS1" = "1" ] ; then
  if [ -z "$RPS1" ] ; then
    RPS1="$KUBIE_PROMPT"
  else
    RPS1="$KUBIE_PROMPT $RPS1"
  fi
else
    PS1="$KUBIE_PROMPT $PS1"
fi

unset KUBIE_PROMPT
"#,
            info.prompt
        )?;
    }

    let mut cmd = Command::new("zsh");
    cmd.env("ZDOTDIR", dir.path());
    info.env_vars.apply(&mut cmd);

    let mut child = cmd.spawn()?;
    child.wait()?;
    Ok(())
}
