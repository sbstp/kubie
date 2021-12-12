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

    write!(temp_rc_file_buf, include_str!("loading.bash"))?;

    if !info.settings.prompt.disable {
        write!(temp_rc_file_buf, include_str!("prompt.bash"), info.prompt)?;
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
