use std::env;
use std::fmt::{self, Display};
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::settings::Settings;

struct Command {
    content: String,
}

impl Command {
    fn new(content: impl Into<String>) -> Command {
        Command {
            content: content.into(),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$({})", self.content)
    }
}

struct Color<D> {
    color: u32,
    content: D,
}

impl<D> Color<D> {
    fn new(color: u32, content: D) -> Color<D> {
        Color { color, content }
    }
}

impl<D> Color<D>
where
    D: Display,
{
    fn isolate<E>(&self, f: &mut fmt::Formatter, content: E) -> fmt::Result
    where
        E: Display,
    {
        write!(f, "\\[{}\\]", content)
    }

    fn start_color(&self, f: &mut fmt::Formatter, color: u32) -> fmt::Result {
        self.isolate(f, format!("\\e[{}m", color))
    }

    fn end_color(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.isolate(f, "\\e[0m")
    }
}

impl<D> fmt::Display for Color<D>
where
    D: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.start_color(f, self.color)?;
        write!(f, "{}", self.content)?;
        self.end_color(f)?;
        Ok(())
    }
}

const RED: u32 = 31;
const GREEN: u32 = 32;
const BLUE: u32 = 34;

/// Generates a PS1 string that shows the current context, namespace and depth.
///
/// Makes sure to protect the escape sequences to that the shell will not count the escape
/// sequences in the length calculation of the prompt.
pub fn generate_ps1(settings: &Settings, depth: u32) -> String {
    let current_exe_path = env::current_exe().expect("could not get own binary path");
    let current_exe_path_str = current_exe_path.to_str().expect("binary path is not unicode");

    let mut parts = vec![];
    parts.push(Color::new(RED, Command::new(format!("{} info ctx", current_exe_path_str))).to_string());
    parts.push(Color::new(GREEN, Command::new(format!("{} info ns", current_exe_path_str))).to_string());
    if settings.prompt.show_depth && depth > 1 {
        parts.push(Color::new(BLUE, depth).to_string());
    }

    format!("[{}]", parts.join("|"))
}

/// Get the current depth of shells.
pub fn get_depth() -> u32 {
    env::var("KUBIE_DEPTH")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

/// Check if we're in a kubie shell.
pub fn is_kubie_active() -> bool {
    let active = env::var("KUBIE_ACTIVE").unwrap_or("0".into());
    return active == "1";
}

/// Ensure that we're inside a kubie shell, returning an error if we aren't.
pub fn ensure_kubie_active() -> Result<()> {
    if !is_kubie_active() {
        return Err(anyhow!("Not in a kubie shell!"));
    }
    Ok(())
}

pub fn get_session_path() -> Option<PathBuf> {
    env::var_os("KUBIE_SESSION").map(PathBuf::from)
}
