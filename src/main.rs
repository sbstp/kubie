use anyhow::Result;
use clap::Parser;

use cmd::meta::Kubie;
use settings::Settings;
use skim::prelude::SkimOptionsBuilder;

mod cmd;
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
    let skim_options = SkimOptionsBuilder::default().multi(false).build().unwrap();
    let kubie = Kubie::parse();

    match kubie {
        Kubie::Context {
            namespace_name,
            context_name,
            kubeconfigs,
            clone,
            recursive,
        } => {
            cmd::context::context(
                &settings,
                &skim_options,
                context_name,
                namespace_name,
                kubeconfigs,
                clone,
                recursive,
            )?;
        }
        Kubie::Namespace {
            namespace_name,
            recursive,
            unset,
        } => {
            cmd::namespace::namespace(&settings, &skim_options, namespace_name, recursive, unset)?;
        }
        Kubie::Info(info) => {
            cmd::info::info(info)?;
        }
        Kubie::Exec {
            context_name,
            namespace_name,
            exit_early,
            context_headers_flag,
            args,
        } => {
            cmd::exec::exec(
                &settings,
                context_name,
                namespace_name,
                exit_early,
                context_headers_flag,
                args,
            )?;
        }
        Kubie::Lint => {
            cmd::lint::lint(&settings)?;
        }
        Kubie::Edit { context_name } => {
            cmd::edit::edit_context(&settings, &skim_options, context_name)?;
        }
        Kubie::EditConfig => {
            cmd::edit::edit_config(&settings)?;
        }
        #[cfg(feature = "update")]
        Kubie::Update => {
            cmd::update::update()?;
        }
        Kubie::Delete { context_name } => {
            cmd::delete::delete_context(&settings, &skim_options, context_name)?;
        }
        Kubie::Export {
            context_name,
            namespace_name,
        } => {
            cmd::export::export(&settings, context_name, namespace_name)?;
        }
    }

    Ok(())
}
