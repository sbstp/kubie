use std::env;
use std::fmt::{self, Display};

use crate::settings::Settings;
use crate::shell::ShellKind;

struct Command {
    content: String,
    shell_kind: ShellKind,
}

impl Command {
    fn new(content: impl Into<String>, shell_kind: ShellKind) -> Command {
        Command {
            content: content.into(),
            shell_kind,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.shell_kind {
            ShellKind::Fish => write!(f, "({})", self.content),
            _ => write!(f, "$({})", self.content),
        }
    }
}

struct Color<D> {
    color: u32,
    content: D,
    shell_kind: ShellKind,
}

impl<D> Color<D> {
    fn new(color: u32, content: D, shell_kind: ShellKind) -> Color<D> {
        Color {
            color,
            content,
            shell_kind,
        }
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
        match self.shell_kind {
            ShellKind::Fish => write!(f, "{}", content),
            ShellKind::Zsh => write!(f, "%{{{}%}}", content),
            _ => write!(f, "\\[{}\\]", content),
        }
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
/// Makes sure to protect the escape sequences so that the shell will not count the escape
/// sequences in the length calculation of the prompt.
pub fn generate_ps1(settings: &Settings, depth: u32, shell_kind: ShellKind) -> String {
    let current_exe_path = env::current_exe().expect("Could not get own binary path");
    let current_exe_path_str = current_exe_path.to_str().expect("Binary path is not unicode");

    let mut parts = vec![];
    parts.push(
        Color::new(
            RED,
            Command::new(format!("{} info ctx", current_exe_path_str), shell_kind),
            shell_kind,
        )
        .to_string(),
    );
    parts.push(
        Color::new(
            GREEN,
            Command::new(format!("{} info ns", current_exe_path_str), shell_kind),
            shell_kind,
        )
        .to_string(),
    );
    if settings.prompt.show_depth && depth > 1 {
        parts.push(Color::new(BLUE, depth, shell_kind).to_string());
    }

    format!("[{}]", parts.join("|"))
}
