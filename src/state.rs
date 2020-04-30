use std::collections::HashMap;
use std::path::PathBuf;

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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct State {
    pub history: HashMap<String, String>,
}

impl State {
    pub fn load() -> Result<State> {
        let path = &*KUBIE_STATE_PATH;
        if !path.exists() {
            return Ok(State::default());
        }
        ioutil::read_json(path)
    }

    pub fn save(&self) -> Result<()> {
        ioutil::write_json(&*KUBIE_STATE_PATH, self)
    }
}
