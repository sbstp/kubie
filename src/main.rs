mod cmd;
mod commands;
mod fzf;
mod kubeconfig;
mod kubectl;
mod tempfile;
mod vars;

use anyhow::Result;
use structopt::StructOpt;

use commands::Kubie;

fn main() -> Result<()> {
    let kubie = Kubie::from_args();

    match kubie {
        Kubie::Context {
            namespace_name,
            context_name,
        } => {
            cmd::context::context(context_name, namespace_name)?;
        }
        Kubie::Namespace { namespace_name } => {
            cmd::namespace::namespace(namespace_name)?;
        }
        Kubie::Info(info) => {
            cmd::info::info(info)?;
        }
        Kubie::Exec {
            context_name,
            namespace_name,
            args,
        } => {
            cmd::exec::exec(context_name, namespace_name, args)?;
        }
        Kubie::Lint => {
            cmd::lint::lint()?;
        }
    }

    Ok(())
}
