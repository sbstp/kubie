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
KUBIE_LOGIN_SHELL=0
if [[ "$OSTYPE" == "darwin"* ]] ; then
    KUBIE_LOGIN_SHELL=1
fi

# Reference for loading behavior
# https://shreevatsa.wordpress.com/2008/03/30/zshbash-startup-files-loading-order-bashrc-zshrc-etc/

if [[ -f "/etc/zshenv" ]] ; then
    source "/etc/zshenv"
fi

if [[ -f "$HOME/.zshenv" ]] ; then
    source "$HOME/.zshenv"
fi

if [[ -f "/etc/zprofile" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "/etc/zprofile"
fi

if [[ -f "$HOME/.zprofile" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "$HOME/.zprofile"
fi

if [[ -f "/etc/zshrc" ]] ; then
    source "/etc/zshrc"
fi

if [[ -f "$HOME/.zshrc" ]] ; then
    source "$HOME/.zshrc"
fi

if [[ -f "/etc/zlogin" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "/etc/zlogin"
fi

if [[ -f "$HOME/.zlogin" && "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    source "$HOME/.zlogin"
fi

autoload -Uz add-zsh-hook

# This function sets the proper KUBECONFIG variable before a command runs,
# in case something overwrote it.
function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

add-zsh-hook preexec __kubie_cmd_pre_exec__
"#,
        )?;

        if !info.settings.prompt.disable {
            write!(
                zshrc,
                r#"
# Activate prompt substitution.
setopt PROMPT_SUBST

# This function fixes the prompt via a precmd hook.
function __kubie_cmd_pre_cmd__() {{
    local KUBIE_PROMPT=$'{}'

    # If KUBIE_ZSH_USE_RPS1 is set, we use RPS1 instead of PS1.
    if [[ "$KUBIE_ZSH_USE_RPS1" == "1" ]] ; then

        # Avoid modifying RPS1 again if the RPS1 has not been reset.
        if [[ "$RPS1" != *"$KUBIE_PROMPT"* ]] ; then

            # If RPS1 is empty, we do not seperate with a space.
            if [[ -z "$RPS1" ]] ; then
                RPS1="$KUBIE_PROMPT"
            else
                RPS1="$KUBIE_PROMPT $RPS1"
            fi
        fi
    else
        # Avoid modifying PS1 again if the PS1 has not been reset.
        if [[ "$PS1" != *"$KUBIE_PROMPT"* ]] ; then
            PS1="$KUBIE_PROMPT $PS1"
        fi
    fi
}}

# When promptinit is activated, a precmd hook which updates PS1 is installed.
# In order to inject the kubie PS1 when promptinit is activated, we must
# also add our own precmd hook which modifies PS1 after promptinit themes.
add-zsh-hook precmd __kubie_cmd_pre_cmd__
"#,
                info.prompt
            )?;
        }
    }

    let mut cmd = Command::new("zsh");
    cmd.env("ZDOTDIR", dir.path());
    info.env_vars.apply(&mut cmd);

    let mut child = cmd.spawn()?;
    child.wait()?;
    Ok(())
}
