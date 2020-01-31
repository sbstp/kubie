use std::fmt::Display;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use anyhow::Context;
use which::which;

pub fn is_available() -> bool {
    !which("fzf").is_err()
}

pub fn select<I, D>(items: I) -> anyhow::Result<Option<String>>
where
    I: IntoIterator<Item = D>,
    D: Display,
{
    let mut child = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Could not spawn fzf")?;

    let stdin = child.stdin.as_mut().expect("stdin not available");
    for item in items {
        writeln!(stdin, "{}", item)?;
    }
    child.wait().context("fzf run failure")?;

    let mut line = String::new();
    child
        .stdout
        .expect("stdout not available")
        .read_to_string(&mut line)
        .context("could not read output from fzf")?;

    let line = line.trim();
    if line.is_empty() {
        Ok(None)
    } else {
        Ok(Some(line.to_string()))
    }
}
