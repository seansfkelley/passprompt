use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions, write};
use std::io::Read;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
  pub last_asked: Option<u64>,
}

impl State {
  pub fn load(p: &PathBuf) -> Result<State, Box<dyn std::error::Error>> {
    let mut state_content = String::new();

    OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(&p)?
      .read_to_string(&mut state_content)?;

    toml::from_str::<State>(&state_content).map_err(toml::de::Error::into)
  }

  pub fn store(&self, p: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let contents = toml::to_string_pretty(&self)?;
    write(p, contents).map_err(std::io::Error::into)
  }
}

pub struct StateManager {
  path: PathBuf,
}

impl StateManager {
  pub fn for_path(path: PathBuf) -> StateManager {
    StateManager { path }
  }

  pub fn with_state<T, F: FnOnce(&mut State) -> Result<T, Box<dyn std::error::Error>>>(
    &self,
    callback: F,
  ) -> Result<T, Box<dyn std::error::Error>> {
    let mut state = State::load(&self.path)?;
    let result = callback(&mut state)?;
    state.store(&self.path)?;
    Ok(result)
  }
}
