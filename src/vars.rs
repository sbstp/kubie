use std::env;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

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
