use std::process::Command;
use anyhow::{Result};

use crate::shell::ShellSpawnInfo;

pub fn spawn_shell(info: &ShellSpawnInfo) -> Result<()> {
    // let dir = tempdir()?;

    let mut cmd = Command::new("nu");
    let mut args = "".to_string();

    for (name, value) in &info.env_vars.vars {
        args.push_str(&format!(r#"let-env {} = '{}';"#, name, value.as_os_str().to_str().unwrap()));

        if String::from("KUBIE_PROMPT_DISABLE").eq(name) && value == "0" {
            let mut _prompt = info.prompt.clone();
            // TODO: This is improvable, but it works for now
            _prompt = _prompt
                .replace("\\[\\e[31m\\]", "")
                .replace("\\[\\e[32m\\]", "")
                .replace("\\[\\e[0m\\]", "")
                .replace("$", "");
            let prompt = format!(
                r#"let-env PROMPT_COMMAND = {{ || $"{prompt}\n(create_left_prompt)" }};"#,
                prompt = _prompt
            );
            args.push_str(prompt.as_str());
        }
    }
    cmd.arg("-e");
    cmd.arg(args);

    let mut child = cmd.spawn()?;
    child.wait()?;
    Ok(())
}
