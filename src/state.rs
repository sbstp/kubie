use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::ioutil;

lazy_static! {
    static ref KUBIE_DATA_DIR: PathBuf = {
        let base_data_dir = dirs::data_local_dir().expect("Could not get local data dir");
        base_data_dir.join("kubie")
    };
    static ref KUBIE_STATE_PATH: PathBuf = KUBIE_DATA_DIR.join("state.json");
}

#[inline]
pub fn path() -> &'static Path {
    &*KUBIE_STATE_PATH
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct State {
    /// This map stores the last namespace in which a context was used, in order to restore the namespace
    /// when the context is entered again.
    ///
    /// The key represents the name of the context and the value is the namespace's name.
    pub namespace_history: HashMap<String, Option<String>>,
}

impl State {
    pub fn load() -> Result<State> {
        if !path().exists() {
            return Ok(State::default());
        }
        ioutil::read_json(path())
    }

    pub fn save(&self) -> Result<()> {
        ioutil::write_json(path(), self)
    }
}
