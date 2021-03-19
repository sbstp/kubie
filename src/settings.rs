use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use glob::glob;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    static ref HOME_DIR: String = dirs::home_dir()
        .expect("could not get home directory path")
        .to_str()
        .expect("home directory contains non unicode characters")
        .to_string();
}

#[inline]
fn home_dir() -> &'static str {
    &*HOME_DIR
}

pub fn expanduser(path: &str) -> String {
    if path.starts_with("~/") {
        format!("{}/{}", home_dir(), &path[2..])
    } else {
        path.to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub configs: Configs,
    #[serde(default)]
    pub prompt: Prompt,
    #[serde(default)]
    pub behavior: Behavior,
}

impl Settings {
    pub fn path() -> String {
        format!("{}/.kube/kubie.yaml", home_dir())
    }

    pub fn load() -> Result<Settings> {
        let settings_path_str = Self::path();
        let settings_path = Path::new(&settings_path_str);

        let mut settings = if settings_path.exists() {
            let file = File::open(settings_path)?;
            let reader = BufReader::new(file);
            serde_yaml::from_reader(reader).context("could not parse kubie config")?
        } else {
            Settings::default()
        };

        // Very important to exclude kubie's own config file ~/.kube/kubie.yaml from the results.
        settings.configs.exclude.push(settings_path_str);
        Ok(settings)
    }

    pub fn get_kube_configs_paths(&self) -> Result<HashSet<PathBuf>> {
        let mut paths = HashSet::new();
        for inc in &self.configs.include {
            let expanded = expanduser(&inc);
            for entry in glob(&expanded)? {
                paths.insert(entry?);
            }
        }

        for exc in &self.configs.exclude {
            let expanded = expanduser(&exc);
            for entry in glob(&expanded)? {
                paths.remove(&entry?);
            }
        }

        Ok(paths)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            shell: Default::default(),
            configs: Configs::default(),
            prompt: Prompt::default(),
            behavior: Behavior::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Configs {
    #[serde(default = "default_include_path")]
    pub include: Vec<String>,
    #[serde(default = "default_exclude_path")]
    pub exclude: Vec<String>,
}

impl Default for Configs {
    fn default() -> Self {
        Configs {
            include: default_include_path(),
            exclude: default_exclude_path(),
        }
    }
}

fn default_include_path() -> Vec<String> {
    let home_dir = home_dir();
    vec![
        format!("{}/.kube/config", home_dir),
        format!("{}/.kube/*.yml", home_dir),
        format!("{}/.kube/*.yaml", home_dir),
        format!("{}/.kube/configs/*.yml", home_dir),
        format!("{}/.kube/configs/*.yaml", home_dir),
        format!("{}/.kube/kubie/*.yml", home_dir),
        format!("{}/.kube/kubie/*.yaml", home_dir),
    ]
}

fn default_exclude_path() -> Vec<String> {
    vec![]
}

#[derive(Debug, Deserialize)]
pub struct Prompt {
    #[serde(default = "def_bool_false")]
    pub disable: bool,
    #[serde(default = "def_bool_true")]
    pub show_depth: bool,
    #[serde(default = "def_bool_false")]
    pub zsh_use_rps1: bool,
    #[serde(default = "def_bool_false")]
    pub fish_use_rprompt: bool,
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            disable: false,
            show_depth: true,
            zsh_use_rps1: false,
            fish_use_rprompt: false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Behavior {
    #[serde(default = "def_bool_true")]
    pub validate_namespaces: bool,
    pub print_context_in_exec: bool,
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior {
            validate_namespaces: true,
            print_context_in_exec: true,
        }
    }
}

fn def_bool_true() -> bool {
    true
}

fn def_bool_false() -> bool {
    false
}

#[test]
fn test_expanduser() {
    assert_eq!(
        expanduser("~/hello/world/*.foo"),
        format!("{}/hello/world/*.foo", home_dir())
    );
}
