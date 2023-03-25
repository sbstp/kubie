use std::process::Command;
use anyhow::{Result};

use crate::shell::ShellSpawnInfo;

pub fn spawn_shell(info: &ShellSpawnInfo) -> Result<()> {
    // let dir = tempdir()?;

    let mut cmd = Command::new("nu");
    let mut args = "".to_string();

    for (name, value) in &info.env_vars.vars {
        args.push_str(&format!(r#"let-env {} = '{}';"#, name, value.as_os_str().to_str().unwrap()));
    }
    cmd.arg("-e");
    cmd.arg(args);

    let mut child = cmd.spawn()?;
    child.wait()?;
    Ok(())
}
