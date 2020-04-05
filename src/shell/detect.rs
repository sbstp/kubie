use std::process::Command;
use std::str;

use anyhow::{anyhow, Result};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShellKind {
    Bash,
    Fish,
    Zsh,
}

impl ShellKind {
    pub fn from_str(name: &str) -> Option<ShellKind> {
        Some(match name {
            "bash" | "dash" => ShellKind::Bash,
            "fish" => ShellKind::Fish,
            "zsh" => ShellKind::Zsh,
            _ => return None,
        })
    }
}

fn run_ps(args: &[&str]) -> Result<Vec<String>> {
    let result = Command::new("ps").args(args).output()?;

    if !result.status.success() {
        let stderr = str::from_utf8(&result.stderr).unwrap_or("Could not decode stderr of ps as utf-8");
        return Err(anyhow!("Error calling ps: {}", stderr));
    }

    let text = str::from_utf8(&result.stdout)?;
    Ok(text.split("\n").filter(|x| !x.is_empty()).map(String::from).collect())
}

fn parent_of(pid: &str) -> Result<String> {
    let lines = run_ps(&["-o", "ppid=", pid])?;
    lines
        .into_iter()
        .next()
        .map(|x| x.trim().to_string())
        .ok_or_else(|| anyhow!("Could not get parent pid of pid={}", pid))
}

fn command_of(pid: &str) -> Result<String> {
    let lines = run_ps(&["-o", "args=", pid])?;
    lines
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Could not get command of pid={}", pid))
}

fn parse_command(cmd: &str) -> &str {
    let first_space = cmd.find(" ").unwrap_or(cmd.len());
    let binary_path = &cmd[..first_space];
    let last_path_sep = binary_path.rfind("/").map(|x| x + 1).unwrap_or(0);
    let binary = &binary_path[last_path_sep..];
    binary
}

pub fn detect() -> Result<ShellKind> {
    let kubie_pid = format!("{}", std::process::id());
    let mut parent_pid = parent_of(&kubie_pid)?;
    loop {
        if parent_pid == "1" {
            return Err(anyhow!("Could not detect shell in use"));
        }

        let cmd = command_of(&parent_pid)?;
        let name = parse_command(&cmd);
        if let Some(kind) = ShellKind::from_str(name) {
            return Ok(kind);
        }

        parent_pid = parent_of(&parent_pid)?;
    }
}

#[test]
fn test_parse_command_simple() {
    assert_eq!(parse_command("bash"), "bash");
}

#[test]
fn test_parse_command_with_args() {
    assert_eq!(parse_command("bash --rcfile hello.sh"), "bash");
}

#[test]
fn test_parse_command_with_path() {
    assert_eq!(parse_command("/bin/bash"), "bash");
}

#[test]
fn test_parse_command_with_path_and_args() {
    assert_eq!(parse_command("/bin/bash --rcfile hello.sh"), "bash");
}
