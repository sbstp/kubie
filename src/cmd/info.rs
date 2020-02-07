use anyhow::Result;

use crate::cmd::meta::{KubieInfo, KubieInfoKind};
use crate::kubeconfig;
use crate::vars;

pub fn info(info: KubieInfo) -> Result<()> {
    match info.kind {
        KubieInfoKind::Context => {
            vars::ensure_kubie_active()?;
            let conf = kubeconfig::get_current_config()?;
            println!("{}", conf.current_context.as_deref().unwrap_or(""));
        }
        KubieInfoKind::Namespace => {
            vars::ensure_kubie_active()?;
            let conf = kubeconfig::get_current_config()?;
            println!("{}", conf.contexts[0].context.namespace);
        }
        KubieInfoKind::Depth => {
            vars::ensure_kubie_active()?;
            println!("{}", vars::get_depth());
        }
    };

    Ok(())
}
