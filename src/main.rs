use anyhow::Result;
use structopt::StructOpt;

use cmd::meta::Kubie;
use settings::Settings;

mod cmd;
mod fzf;
mod ioutil;
mod kubeconfig;
mod kubectl;
mod session;
mod settings;
mod shell;
mod state;
mod vars;

fn main() -> Result<()> {
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
        Kubie::Update => {
            cmd::update::update()?;
        }
        Kubie::Delete { context_name } => {
            cmd::delete::delete_context(&settings, context_name)?;
        }
    }

    Ok(())
}
