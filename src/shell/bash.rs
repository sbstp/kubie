use std::io::Write;
use std::process::Command;

use anyhow::Result;

use super::ShellInfo;
use crate::tempfile::Tempfile;
use crate::vars;

pub fn spawn_shell(info: &ShellInfo) -> Result<()> {
    let mut temp_rc_file = Tempfile::new("/tmp", "kubie-bashrc-", ".bash")?;
    write!(
        temp_rc_file,
        r#"
if [ -f "$HOME/.bashrc" ] ; then
    source "$HOME/.bashrc"
elif [ -f "/etc/skel/.bashrc" ] ; then
    source /etc/skel/.bashrc
fi

function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

trap '__kubie_cmd_pre_exec__' DEBUG

KUBIE_PROMPT='{}'
PS1="$KUBIE_PROMPT $PS1"
unset KUBIE_PROMPT
"#,
        vars::generate_ps1(info.settings, info.next_depth),
    )?;

    let mut child = Command::new("bash")
        .arg("--rcfile")
        .arg(temp_rc_file.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", info.next_depth.to_string())
        .env("KUBIE_KUBECONFIG", info.temp_config_file.path())
        .env("KUBIE_SESSION", info.temp_session_file.path())
        .spawn()?;
    child.wait()?;

    Ok(())
}
