use std::io::Write;
use std::process::Command;

use anyhow::Result;

use super::ShellSpawnInfo;

pub fn spawn_shell(info: &ShellSpawnInfo) -> Result<()> {
    let mut temp_rc_file = tempfile::Builder::new()
        .prefix("kubie-bashrc")
        .suffix(".bash")
        .tempfile()?;

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
"#
    )?;

    if !info.settings.prompt.disable {
        write!(
            temp_rc_file,
            r#"
KUBIE_PROMPT='{}'
PS1="$KUBIE_PROMPT $PS1"
unset KUBIE_PROMPT
"#,
            info.prompt,
        )?;
    }

    temp_rc_file.flush()?;

    let mut cmd = Command::new("bash");
    cmd.arg("--rcfile");
    cmd.arg(temp_rc_file.path());
    info.env_vars.apply(&mut cmd);

    let mut child = cmd.spawn()?;
    child.wait()?;

    Ok(())
}
