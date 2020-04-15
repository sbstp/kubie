use std::io::{BufWriter, Write};
use std::process::Command;

use anyhow::Result;

use super::ShellSpawnInfo;

pub fn spawn_shell(info: &ShellSpawnInfo) -> Result<()> {
    let temp_rc_file = tempfile::Builder::new()
        .prefix("kubie-bashrc")
        .suffix(".bash")
        .tempfile()?;
    let mut temp_rc_file_buf = BufWriter::new(temp_rc_file.as_file());

    write!(
        temp_rc_file_buf,
        r#"
KUBIE_LOGIN_SHELL=0
if [[ "$OSTYPE" == "darwin"* ]] ; then
    KUBIE_LOGIN_SHELL=1
fi

# Reference for loading behavior
# https://shreevatsa.wordpress.com/2008/03/30/zshbash-startup-files-loading-order-bashrc-zshrc-etc/


if [[ "$KUBIE_LOGIN_SHELL" == "1" ]] ; then
    if [[ -f "/etc/profile" ]] ; then
        source "/etc/profile"
    fi

    if [[ -f "$HOME/.bash_profile" ]] ; then
        source "$HOME/.bash_profile"
    elif [[ -f "$HOME/.bash_login" ]] ; then
        source "$HOME/.bash_login"
    elif [[ -f "$HOME/.profile" ]] ; then
        source "$HOME/.profile"
    fi
else
    if [[ -f "/etc/bash.bashrc" ]] ; then
        source "/etc/bash.bashrc"
    fi

    if [[ -f "$HOME/.bashrc" ]] ; then
        source "$HOME/.bashrc"
    fi
fi

function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

trap '__kubie_cmd_pre_exec__' DEBUG
"#
    )?;

    if !info.settings.prompt.disable {
        write!(
            temp_rc_file_buf,
            r#"
KUBIE_PROMPT='{}'
PS1="$KUBIE_PROMPT $PS1"
unset KUBIE_PROMPT
"#,
            info.prompt,
        )?;
    }

    temp_rc_file_buf.flush()?;

    let mut cmd = Command::new("bash");
    cmd.arg("--rcfile");
    cmd.arg(temp_rc_file.path());
    info.env_vars.apply(&mut cmd);

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
