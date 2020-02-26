use std::io::Write;
use std::process::Command;

use anyhow::Result;

use crate::kubeconfig::KubeConfig;
use crate::session::Session;
use crate::settings::Settings;
use crate::tempfile::Tempfile;
use crate::vars;

pub fn spawn_shell(settings: &Settings, config: KubeConfig, session: &Session) -> Result<()> {
    let temp_config_file = Tempfile::new("/tmp", "kubie-config", ".yaml")?;
    config.write_to(&*temp_config_file)?;

    let temp_session_file = Tempfile::new("/tmp", "kubie-session", ".yaml")?;
    session.save(Some(temp_session_file.path()))?;

    let depth = vars::get_depth();
    let next_depth = depth + 1;

    let mut temp_rc_file = Tempfile::new("/tmp", "kubie-bashrc-", ".bash")?;
    write!(
        temp_rc_file,
        r#"
if [ -f "$HOME/.bashrc" ] ; then
    source "$HOME/.bashrc"
elif [ -f "/etc/skel/.bashrc" ] ; then
    source /etc/skel/.bashrc
fi

function kubectx {{
    kubie ctx "$@"
}}

function kubens {{
    kubie ns "$@"
}}

function k {{
    echo "k on disabled to prevent misuse."
}}

function __kubie_cmd_pre_exec__() {{
    export KUBECONFIG="$KUBIE_KUBECONFIG"
}}

trap '__kubie_cmd_pre_exec__' DEBUG

PROMPT='{}'
export PS1="$PROMPT ${{PS1}}"
unset PROMPT
"#,
        vars::generate_ps1(settings, next_depth),
    )?;

    let mut child = Command::new("bash")
        .arg("--rcfile")
        .arg(temp_rc_file.path())
        .env("KUBIE_ACTIVE", "1")
        .env("KUBIE_DEPTH", next_depth.to_string())
        .env("KUBIE_KUBECONFIG", temp_config_file.path())
        .env("KUBIE_SESSION", temp_session_file.path())
        .spawn()?;
    child.wait()?;

    Ok(())
}
