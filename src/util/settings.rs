use std::vec::Vec;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::level::LevelId;

#[derive(Serialize, Deserialize)]
struct GameFile {
  title: String,
  authors: Vec<String>,
  starting_level: LevelId
}

pub fn gen_default_game_file() -> bool {
  let default = GameFile {
    title: String::from("Temple"),
    authors: vec!(String::from("ALUMUX (Chris M.)")),
    starting_level: 0
  };

  let toml = toml::to_string_pretty(&default).unwrap();
  fs::write("assets/game.toml", toml).is_ok()
}
