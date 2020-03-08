use std::env;
use std::fs;
use std::fs::Permissions;
use std::os::unix::prelude::*;
use std::path::Path;

use crate::tempfile::Tempfile;
use anyhow::Result;
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const RELEASES_LIST: &str = "https://api.github.com/repos/sbstp/kubie/contents/releases/linux?ref=master";
const RELEASE_BASE_URL: &str = "https://github.com/sbstp/kubie/raw/master/releases/linux/amd64";
const FILENAME: &str = "kubie";

#[derive(Debug, Deserialize)]
struct TreeUrl {
    git_url: String,
}

#[derive(Debug, Deserialize)]
struct Tree {
    tree: Vec<KubieVersion>,
}

#[derive(Debug, Deserialize)]
struct KubieVersion {
    path: String,
}

pub fn update() -> Result<()> {
    let latest_version = get_latest_version()?;
    if latest_version.eq(&format!("v{}", VERSION)) {
        println!("Kubie is up-to-date : v{}", VERSION);
    } else {
        println!(
            "A new version of Kubie is available ({}), the new version will be automatically installed...",
            latest_version
        );
        let resp = attohttpc::get(format!("{}/{}/{}", RELEASE_BASE_URL, latest_version, FILENAME)).send()?;
        if resp.is_success() {
            let tmp_file = Tempfile::new("/tmp", "kubie", "")?;
            resp.write_to(&*tmp_file)?;
            let old_file = env::current_exe().expect("could not get own binary path");
            let res = replace_file(&old_file, tmp_file.path());
            match res {
                Ok(_) => {
                    println!(
                        "Kubie has been updated successfully. Enjoy :) ({})",
                        Path::display(&old_file)
                    );
                }
                Err(err) => {
                    println!("Updated failed... {}", err);
                }
            }
        }
    }
    Ok(())
}

pub fn get_latest_version() -> Result<String> {
    let tree_url: Vec<TreeUrl> = attohttpc::get(&RELEASES_LIST).send()?.json()?;
    let tree: Tree = attohttpc::get(&tree_url[0].git_url).send()?.json()?;
    let latest_version = &tree.tree.last().unwrap().path;
    Ok(latest_version.to_string())
}

pub fn replace_file(old_file: &Path, new_file: &Path) -> std::io::Result<()> {
    fs::set_permissions(new_file, Permissions::from_mode(0o755))?;
    fs::rename(&new_file, old_file)?;
    Ok(())
}
