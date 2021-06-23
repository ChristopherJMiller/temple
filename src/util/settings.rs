use std::vec::Vec;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::level::LevelId;
use crate::util::files::GAME_SETTING_PATH;

#[derive(Serialize, Deserialize)]
pub struct GameFile {
  pub title: String,
  pub authors: Vec<String>,
  pub starting_level: LevelId
}

pub fn gen_default_game_file() -> bool {
  let default = GameFile {
    title: String::from("Temple"),
    authors: vec!(String::from("ALUMUX (Chris M.)")),
    starting_level: 0
  };

  let toml = toml::to_string_pretty(&default).unwrap();
  fs::write(GAME_SETTING_PATH, toml).is_ok()
}

pub fn get_game_file() -> GameFile {
  if let Ok(file) = fs::read_to_string(GAME_SETTING_PATH) {
    match toml::from_str::<GameFile>(file.as_str()) {
      Ok(game_file) => game_file,
      Err(err) => panic!("Error while loading game file: {}", err)
    }
  } else {
    panic!(format!("Failed to find game file at path {}", GAME_SETTING_PATH));
  }
}
