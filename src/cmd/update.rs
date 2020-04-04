use std::env;
use std::fs;
use std::fs::Permissions;
use std::os::unix::prelude::*;
use std::path::Path;

use crate::tempfile::Tempfile;
use anyhow::{anyhow, Context, Result};
use os_info::{Info, Type};
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/sbstp/kubie/releases/latest";
const FILENAME: &str = "kubie";

#[derive(Debug, Deserialize)]
pub struct Release {
    tag_name: String,
    prerelease: bool,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
    state: String,
}

pub fn update() -> Result<()> {
    let latest_release: Release = get_latest_release()?;
    let latest_version = latest_release.tag_name;
    if latest_version.eq(&format!("v{}", VERSION)) {
        println!("Kubie is up-to-date : v{}", VERSION);
    } else {
        println!(
            "A new version of Kubie is available ({}), the new version will be automatically installed...",
            latest_version
        );
        println!(
            "Downloading at {} or {}",
            latest_release.assets[0].browser_download_url, latest_release.assets[1].browser_download_url
        );
        let mut linux_download_url = String::new();
        let mut macos_download_url = String::new();
        for asset in latest_release.assets {
            if asset.browser_download_url.contains("linux-amd64") {
                linux_download_url = asset.browser_download_url;
            } else if asset.browser_download_url.contains("darwin-amd64") {
                macos_download_url = asset.browser_download_url;
            }
        }
        let mut download_url = String::new();
        match os_info::get().os_type() {
            os_info::Type::Macos => {
                if &macos_download_url != "" {
                    download_url = macos_download_url;
                } else {
                    return Err(anyhow!("Sorry, this release has no build for OSX, please create an issue : https://github.com/sbstp/kubie/issues"));
                }
            }
            os_info::Type::Windows => {
                println!("Your operating system is not supported.");
            }
            _ => {
                //The fallback is Linux
                if &linux_download_url != "" {
                    download_url = linux_download_url;
                } else {
                    return Err(anyhow!("Sorry, this release has no build for Linux, please create an issue : https://github.com/sbstp/kubie/issues"));
                }
            }
        }
        let resp = attohttpc::get(download_url).send()?;
        if resp.is_success() {
            let tmp_file = Tempfile::new("/tmp", "kubie", "")?;
            resp.write_to(&*tmp_file)?;
            let old_file = env::current_exe().expect("could not get own binary path");
            replace_file(&old_file, tmp_file.path()).context("Update failed. Consider using sudo?")?;
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
    fs::rename(&new_file, old_file)?;
    Ok(())
}
