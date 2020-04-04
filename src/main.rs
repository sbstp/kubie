mod cmd;
mod fzf;
mod kubeconfig;
mod kubectl;
mod session;
mod settings;
mod shell;
mod tempfile;
mod vars;

use anyhow::Result;
use structopt::StructOpt;

use cmd::meta::Kubie;
use settings::Settings;

fn main() -> Result<()> {
    println!("Shell: {:?}", shell::detect()?);

    let settings = Settings::load()?;
    let kubie = Kubie::from_args();

    match kubie {
        Kubie::Context {
            namespace_name,
            context_name,
            recursive,
        } => {
            cmd::context::context(&settings, context_name, namespace_name, recursive)?;
        }
        Kubie::Namespace {
            namespace_name,
            recursive,
        } => {
            cmd::namespace::namespace(&settings, namespace_name, recursive)?;
        }
        Kubie::Info(info) => {
            cmd::info::info(info)?;
        }
        Kubie::Exec {
            context_name,
            namespace_name,
            exit_early,
            args,
        } => {
            cmd::exec::exec(&settings, context_name, namespace_name, exit_early, args)?;
        }
        Kubie::Lint => {
            cmd::lint::lint(&settings)?;
        }
        Kubie::Edit { context_name } => {
            cmd::edit::edit_context(&settings, context_name)?;
        }
        Kubie::EditConfig => {
            cmd::edit::edit_config()?;
        }
    }

    Ok(())
}
