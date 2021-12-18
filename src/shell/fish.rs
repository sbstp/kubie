use std::process::Command;

use anyhow::Result;

use super::ShellSpawnInfo;

pub fn spawn_shell(info: &ShellSpawnInfo) -> Result<()> {
    let mut cmd = Command::new("fish");
    // run fish as an interactive login shell
    cmd.arg("-ilC");
    cmd.arg(format!(include_str!("prompt.fish"), prompt = info.prompt));
    info.env_vars.apply(&mut cmd);

    let mut child = cmd.spawn()?;
    child.wait()?;
    Ok(())
}
