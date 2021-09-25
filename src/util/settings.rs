use std::fs;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::util::files::GAME_SETTING_PATH;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameFile {
  pub title: String,
  pub authors: Vec<String>,
}

impl Default for GameFile {
  fn default() -> Self {
    Self {
      title: String::from("Temple"),
      authors: vec![String::from("ALUMUX (Chris M.)")],
    }
  }
}

pub struct Version(pub String);

/// Generates a default game file
#[allow(dead_code)]
pub fn gen_default_game_file() -> bool {
  let toml = toml::to_string_pretty(&GameFile::default()).unwrap();
  fs::write(GAME_SETTING_PATH, toml).is_ok()
}

pub fn get_game_file() -> GameFile {
  if let Ok(file) = fs::read_to_string(GAME_SETTING_PATH) {
    match toml::from_str::<GameFile>(file.as_str()) {
      Ok(game_file) => game_file,
      Err(err) => panic!("Error while loading game file: {}", err),
    }
  } else {
    panic!("Failed to find game file at path {}", GAME_SETTING_PATH);
  }
}
