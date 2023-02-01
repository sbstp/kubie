use std::env;
use std::fs;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::{Context, Result};
use cfg_if::cfg_if;
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/sbstp/kubie/releases/latest";

#[derive(Debug, Deserialize)]
pub struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

impl Release {
    pub fn get_latest() -> Result<Release> {
        let latest_release = attohttpc::get(LATEST_RELEASE_URL).send()?.json()?;
        Ok(latest_release)
    }

    // Get the right binary name based on which OS and architecture kubie was built-on.
    // Names match the GitHub releases: https://github.com/sbstp/kubie/releases
    fn get_binary_name() -> Option<&'static str> {
        cfg_if! {
            if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
                Some("kubie-linux-amd64")
            } else if #[cfg(all(target_os = "linux", target_arch = "arm"))] {
                Some("kubie-linux-arm32")
            } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
                Some("kubie-linux-arm64")
            } else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
                Some("kubie-darwin-amd64")
            } else if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
                Some("kubie-darwin-arm64")
            } else {
                None
            }
        }
    }

    pub fn get_binary_url(&self) -> Option<&str> {
        let binary_name = Self::get_binary_name()?;

        for asset in self.assets.iter() {
            if asset.browser_download_url.contains(binary_name) {
                return Some(&asset.browser_download_url);
            }
        }
        None
    }
}

#[derive(Debug, Deserialize)]
struct Asset {
    browser_download_url: String,
}

pub fn update() -> Result<()> {
    let latest_release = Release::get_latest()?;
    if latest_release.tag_name == format!("v{VERSION}") {
        println!("Kubie is up-to-date : v{VERSION}");
    } else {
        println!(
            "A new version of Kubie is available ({}), the new version will be installed by replacing this binary.",
            latest_release.tag_name
        );

        let download_url = latest_release.get_binary_url().context("Sorry, this release has no build for your OS, please create an issue : https://github.com/sbstp/kubie/issues")?;
        println!("Download url is: {download_url}");

        let resp = attohttpc::get(download_url).send()?;
        if resp.is_success() {
            let temp_file = tempfile::Builder::new().prefix("kubie").tempfile()?;
            resp.write_to(&temp_file)?;

            let old_file = env::current_exe().expect("Could not get own binary path");
            replace_file(&old_file, temp_file.path()).context("Update failed. Consider using sudo?")?;

            println!("Kubie has been updated successfully: {}", Path::display(&old_file));
        }
    }
    Ok(())
}

pub fn replace_file(old_file: &Path, new_file: &Path) -> std::io::Result<()> {
    fs::set_permissions(new_file, Permissions::from_mode(0o755))?;
    fs::remove_file(old_file)?;
    fs::copy(new_file, old_file)?;
    Ok(())
}

#[test]
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn test_binary_name() {
    assert_eq!(Release::get_binary_name(), Some("kubie-linux-amd64"))
}

#[test]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn test_binary_name() {
    assert_eq!(Release::get_binary_name(), Some("kubie-darwin-amd64"))
}

#[test]
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn test_binary_name() {
    assert_eq!(Release::get_binary_name(), Some("kubie-darwin-arm64"))
}
