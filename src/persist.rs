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
    static ref KUBIE_PERSIST_PATH: PathBuf = { KUBIE_DATA_DIR.join("persist.json") };
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Persist {
    pub history: HashMap<String, String>,
}

impl Persist {
    pub fn load() -> Result<Persist> {
        ioutil::read_json(&*KUBIE_PERSIST_PATH)
    }

    pub fn save(&self) -> Result<()> {
        ioutil::write_json(&*KUBIE_PERSIST_PATH, self)
    }
}
