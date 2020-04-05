use std::env;
use std::fs;
use std::fs::Permissions;
use std::os::unix::prelude::*;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/sbstp/kubie/releases/latest";

#[derive(Debug, Deserialize)]
pub struct Release {
    tag_name: String,
    prerelease: bool,
    assets: Vec<Asset>,
}

impl Release {
    fn get_linux_binary_url(&self) -> Option<&str> {
        for asset in self.assets.iter() {
            if asset.browser_download_url.contains("linux-amd64") {
                return Some(&asset.browser_download_url);
            }
        }
        None
    }

    fn get_macos_binary_url(&self) -> Option<&str> {
        for asset in self.assets.iter() {
            if asset.browser_download_url.contains("darwin-amd64") {
                return Some(&asset.browser_download_url);
            }
        }
        None
    }

    pub fn get_binary_url(&self) -> Option<&str> {
        match os_info::get().os_type() {
            os_info::Type::Macos => {
                return self.get_macos_binary_url();
            }
            os_info::Type::Windows => None,
            _ => {
                return self.get_linux_binary_url();
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
    state: String,
}

pub fn update() -> Result<()> {
    let latest_release: Release = get_latest_release()?;
    if latest_release.tag_name == format!("v{}s", VERSION) {
        println!("Kubie is up-to-date : v{}", VERSION);
    } else {
        println!(
            "A new version of Kubie is available ({}), the new version will be automatically installed...",
            latest_release.tag_name
        );
        let download_url = latest_release.get_binary_url().context("Sorry, this release has no build for your OS, please create an issue : https://github.com/sbstp/kubie/issues")?;
        let resp = attohttpc::get(download_url).send()?;
        if resp.is_success() {
            let temp_file = tempfile::Builder::new().prefix("kubie").tempfile()?;
            resp.write_to(&temp_file)?;

            let old_file = env::current_exe().expect("Could not get own binary path");
            replace_file(&old_file, temp_file.path()).context("Update failed. Consider using sudo?")?;

            println!(
                "Kubie has been updated successfully. Enjoy :) ({})",
                Path::display(&old_file)
            );
        }
    }
    Ok(())
}

pub fn get_latest_release() -> Result<Release> {
    let latest_release = attohttpc::get(&LATEST_RELEASE_URL).send()?.json()?;
    Ok(latest_release)
}

pub fn replace_file(old_file: &Path, new_file: &Path) -> std::io::Result<()> {
    fs::set_permissions(new_file, Permissions::from_mode(0o755))?;
    fs::remove_file(old_file)?;
    fs::copy(&new_file, old_file)?;
    Ok(())
}
