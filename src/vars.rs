use std::env;
use std::fmt::{self, Display};

use anyhow::{anyhow, Context, Result};

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
pub fn generate_ps1(depth: u32) -> String {
    format!(
        "[{}|{}|{}]",
        Color::new(RED, Command::new("kubie info ctx")),
        Color::new(GREEN, Command::new("kubie info ns")),
        Color::new(BLUE, depth),
    )
}

/// Generates a PATH variable which contains the directory inside of which kubie resides.
///
/// This is required by the PS1 variable which makes calls to kubie to display information.
/// This function also makes sure to not insert the directory again in the PATH to avoid
/// wasteful growth of the PATH variable.
///
/// The downside of this function is that it requires the PATH to be unicode.
pub fn generate_path() -> Result<String> {
    let path = match env::var("PATH") {
        Ok(path) => path,
        Err(env::VarError::NotPresent) => "".into(),
        Err(env::VarError::NotUnicode { .. }) => return Err(anyhow!("PATH variable contains non unicode bytes")),
    };
    let kubie_exe = env::current_exe().context("Could not get current exe path")?;
    let kubie_exe = kubie_exe
        .canonicalize()
        .context("Could not get absolute path of current exe")?;
    let kubie_dir = kubie_exe.parent().expect("Kubie executable has not parent directory");
    let kubie_dir = kubie_dir
        .to_str()
        .context("Kubie parent folder contains non unicode bytes")?;

    let mut directories: Vec<_> = path.split(':').collect();

    if !directories.contains(&kubie_dir) {
        directories.insert(0, kubie_dir);
    }

    Ok(directories.join(":"))
}
