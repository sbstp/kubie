use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, prelude::*};
use std::process::Command;

fn main() -> io::Result<()> {
    let shell = env::var_os("SHELL").unwrap_or("/bin/bash".into());
    let path = env::var_os("PATH").unwrap();

    let mut rcfile = File::create("/tmp/kubie.sh")?;
    write!(
        rcfile,
        r#"\
if [ -f "$HOME/.bashrc" ] ; then
    source "$HOME/.bashrc"
fi

PS1="[\e[0;32m$(ls)\e[m|\e[0;31m$(ls)\e[m] ${{PS1}}"
"#
    )?;

    let mut new_path = OsString::new();
    new_path.push(env::current_exe().unwrap().parent().unwrap());
    new_path.push(":");
    new_path.push(path);

    let mut child = Command::new(shell)
        .arg("--rcfile")
        .arg("/tmp/kubie.sh")
        .env("KUBECONFIG", "hello")
        .env("PATH", new_path)
        // create bashrc file to overwrite PS1
        // use --rcfile when spawning shell
        .spawn()?;
    child.wait()?;

    Ok(())
}
