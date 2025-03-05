use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::ioutil;
use crate::vars;

/// Session contains information which is scoped to a kubie shell.
///
/// Currently it stores the history of contexts and namespaces entered to allow
/// users to switch back to the previous context with `-`.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Session {
    history: Vec<HistoryEntry>,
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

        ioutil::read_json(session_path)
    }

    pub fn save(&self, path: Option<&Path>) -> Result<()> {
        let session_path = match path {
            Some(p) => p.to_path_buf(),
            None => vars::get_session_path().context("KUBIE_SESSION env variable missing")?,
        };

        ioutil::write_json(session_path, self)
    }

    pub fn add_history_entry(&mut self, context: impl Into<String>, namespace: Option<impl Into<String>>) {
        self.history.push(HistoryEntry {
            context: context.into(),
            namespace: namespace.map(Into::into),
        })
    }

    pub fn get_previous_context(&self) -> Option<&HistoryEntry> {
        let current_context = self.history.last()?;
        self.history
            .iter()
            .rev()
            .skip(1)
            .find(|&entry| current_context.context != entry.context)
    }

    pub fn get_last_context(&self) -> Option<&HistoryEntry> {
        return self.history.last()
    }

    pub fn get_last_namespace(&self) -> Option<&str> {
        let current_context = self.history.last()?;
        for entry in self.history.iter().rev().skip(1) {
            if current_context.context != entry.context {
                return None;
            }
            if current_context.namespace != entry.namespace {
                return entry.namespace.as_deref();
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryEntry {
    pub context: String,
    pub namespace: Option<String>,
}
