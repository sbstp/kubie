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
if [[ -f "$HOME/.zshrc" ]] ; then
    source "$HOME/.zshrc"
fi

autoload -Uz add-zsh-hook

# This function sets the proper KUBECONFIG variable before a command runs,
# in case something overwrote it.
function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

add-zsh-hook preexec __kubie_cmd_pre_exec__

# Check if prompt is disabled.
if [[ "$KUBIE_PROMPT_DISABLE" != "1" ]] ; then
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
fi
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
