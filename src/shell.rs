use std::io::Write;
use std::process::Command;

use anyhow::Result;

use crate::kubeconfig::KubeConfig;
use crate::settings::Settings;
use crate::tempfile::Tempfile;
use crate::vars;

pub fn spawn_shell(settings: &Settings, config: KubeConfig) -> Result<()> {
    let temp_config_file = Tempfile::new("/tmp", "kubie-config", ".yaml")?;
    config.write_to(&*temp_config_file)?;

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
    echo "kubectx disabled to prevent misuse."
}}

function kubens {{
    echo "kubens disabled to prevent misuse."
}}

function k {{
    echo "k on disabled to prevent misuse."
}}

function kubectl {{
    KUBECONFIG="${{KUBIE_KUBECONFIG}}" $(which kubectl) "$@"
}}

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
        .spawn()?;
    child.wait()?;

    Ok(())
}
