use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use fs2::FileExt;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::ioutil;

lazy_static! {
    static ref KUBIE_DATA_DIR: PathBuf = {
        let base_data_dir = dirs::data_local_dir().expect("Could not get local data dir");
        base_data_dir.join("kubie")
    };
    static ref KUBIE_STATE_PATH: PathBuf = KUBIE_DATA_DIR.join("state.json");
    static ref KUBIE_STATE_LOCK_PATH: PathBuf = KUBIE_DATA_DIR.join(".state.json.lock");
}

#[inline]
pub fn path() -> &'static Path {
    &*KUBIE_STATE_PATH
}

#[inline]
fn lock_path() -> &'static Path {
    &*KUBIE_STATE_LOCK_PATH
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
    /// Loads the state.json from the filesystem, waiting for a file lock to ensure no other
    /// concurrent Kubie processes are accessing/writing the file at the same time.
    pub fn load() -> Result<State> {
        Self::access(|state| Ok(state))
    }

    /// Takes a closure that allows for modifications of the state. Automatically handles
    /// locking/unlocking and saving after execution of the closure.
    pub fn modify<F: FnOnce(&mut State) -> Result<()>>(func: F) -> Result<()> {
        Self::access(|mut state| {
            func(&mut state)?;
            state.save()?;
            Ok(())
        })
    }

    fn access<R, F: FnOnce(State) -> Result<R>>(func: F) -> Result<R> {
        let path = KUBIE_STATE_LOCK_PATH.display();

        // Acquire the lock
        let flock = File::create(lock_path())?;
        flock
            .lock_exclusive()
            .with_context(|| format!("Failed to lock state: {}", path))?;

        // Do the work
        let state = State::read_and_parse()
            .with_context(|| format!("Could not load state file: {}", KUBIE_STATE_PATH.display()));
        let result = match state {
            Ok(s) => func(s),
            Err(e) => Err(e),
        };

        // Release the lock
        flock
            .unlock()
            .with_context(|| format!("Failed to unlock state: {}", path))?;
        result
    }

    fn read_and_parse() -> Result<State> {
        if !path().exists() {
            return Ok(State::default());
        }
        ioutil::read_json(path()).with_context(|| format!("Failed to read state from '{}'", KUBIE_STATE_PATH.display()))
    }

    fn save(&self) -> Result<()> {
        ioutil::write_json(path(), self)
            .with_context(|| format!("Failed to write state to '{}'", KUBIE_STATE_PATH.display()))
    }
}
