use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::vars;

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    history: Vec<HistoryEntry>,
}

impl Default for Session {
    fn default() -> Self {
        Session { history: Vec::new() }
    }
}

impl Session {
    pub fn load() -> Result<Session> {
        let session_path = match vars::get_session_path() {
            None => return Ok(Default::default()),
            Some(x) => x,
        };

        if !session_path.exists() {
            return Ok(Default::default());
        }

        let file = File::open(session_path)?;
        let reader = BufReader::new(file);
        let sess = serde_yaml::from_reader(reader)?;
        Ok(sess)
    }

    pub fn save(&self, path: Option<&Path>) -> Result<()> {
        let session_path = match path {
            Some(p) => p.to_path_buf(),
            None => vars::get_session_path().context("KUBIE_SESSION env variable missing")?,
        };

        let file = File::create(session_path)?;
        let writer = BufWriter::new(file);

        serde_yaml::to_writer(writer, self)?;

        Ok(())
    }

    pub fn add_history_entry(&mut self, context: impl Into<String>, namespace: impl Into<String>) {
        self.history.push(HistoryEntry {
            context: context.into(),
            namespace: namespace.into(),
        })
    }

    pub fn get_last_context(&self) -> Option<&HistoryEntry> {
        let current_context = self.history.last()?;
        for entry in self.history.iter().rev().skip(1) {
            if current_context.context != entry.context {
                return Some(entry);
            }
        }
        None
    }

    pub fn get_last_namespace(&self) -> Option<&str> {
        let current_context = self.history.last()?;
        for entry in self.history.iter().rev().skip(1) {
            if current_context.context != entry.context {
                return None;
            }
            if current_context.namespace != entry.namespace {
                return Some(&entry.namespace);
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryEntry {
    pub context: String,
    pub namespace: String,
}
