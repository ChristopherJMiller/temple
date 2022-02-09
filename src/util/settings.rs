//! Handles the main settings file, `game.toml`.

use std::fs;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

use super::files::from_game_root;
use crate::util::files::GAME_SETTING_PATH;

/// Describes what method should be used to allow players to move from one level
/// to the next.
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LevelTransistionType {
  /// On play, the player is loaded into the overworld to be able to access
  /// levels. Completing levels will bring them back to the overworld.
  Overworld,

  /// On play, the player is loaded into the first level. Completing levels will
  /// bring them to the next level. If used, `level_order` must be supplied in
  /// `game.toml`
  NoOverworld,
}

/// Object that represents `game.toml`.
#[derive(Clone, Serialize, Deserialize)]
pub struct GameFile {
  /// Title of the game
  pub title: String,

  /// Authors of the game
  pub authors: Vec<String>,

  /// Type of Level Transistion used
  pub level_transistion: LevelTransistionType,

  /// If [LevelTransistionType::NoOverworld] is used, this defines the level
  /// order of the game.
  pub level_order: Option<Vec<u32>>,

  /// Game Credits, parsed as markdown.
  pub credits: String,

  /// Path to music for credits
  pub credit_music: String,
}

impl Default for GameFile {
  fn default() -> Self {
    Self {
      title: String::from("Temple"),
      authors: vec![String::from("ALUMUX (Chris M.)")],
      level_transistion: LevelTransistionType::NoOverworld,
      level_order: Some(vec![0]),
      credits: Default::default(),
      credit_music: Default::default(),
    }
  }
}

/// Loads `game.toml`.
pub fn get_game_file() -> GameFile {
  if let Ok(file) = fs::read_to_string(from_game_root(GAME_SETTING_PATH)) {
    match toml::from_str::<GameFile>(file.as_str()) {
      Ok(game_file) => game_file,
      Err(err) => panic!("Error while loading game file: {}", err),
    }
  } else {
    panic!("Failed to find game file at path {}", GAME_SETTING_PATH);
  }
}

#[cfg(test)]
mod tests {
  use crate::util::settings::*;
  use crate::util::files::*;
  use std::fs;

  #[test]
  #[should_panic]
  fn test_game_file_load() {
    fs::create_dir_all(from_game_root(ASSET_PATH)).unwrap();

    let game_file = GameFile::default();
    fs::write(from_game_root(GAME_SETTING_PATH), toml::to_string_pretty(&game_file).unwrap()).unwrap();

    let read_game_file = get_game_file();
    assert_eq!(game_file.title, read_game_file.title);

    fs::remove_file(from_game_root(GAME_SETTING_PATH)).unwrap();

    get_game_file();
  }
}
