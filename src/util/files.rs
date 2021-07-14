use std::fs;

use toml::de::Error;

use crate::level::LevelFile;
use crate::sprite::{SpriteFile, SpriteTypesFile};
use crate::util::settings::GameFile;

pub const GAME_SETTING_PATH: &str = "assets/game.toml";
pub const LEVEL_FILE_PATH: &str = "assets/levels.toml";
pub const SPRITE_FILE_PATH: &str = "assets/sprites/sprites.toml";
pub const SPRITE_TYPE_FILE_PATH: &str = "assets/sprites/types.toml";

pub fn verify_files() {
  let game_settings_file =
    fs::read_to_string(GAME_SETTING_PATH).unwrap_or_else(|_| panic!("Failed to open file {}", GAME_SETTING_PATH));
  let level_file =
    fs::read_to_string(LEVEL_FILE_PATH).unwrap_or_else(|_| panic!("Failed to open file {}", LEVEL_FILE_PATH));
  let sprite_file =
    fs::read_to_string(SPRITE_FILE_PATH).unwrap_or_else(|_| panic!("Failed to open file {}", SPRITE_FILE_PATH));
  let sprite_types_file = fs::read_to_string(SPRITE_TYPE_FILE_PATH)
    .unwrap_or_else(|_| panic!("Failed to open file {}", SPRITE_TYPE_FILE_PATH));

  let verify_game_settings = toml::from_str::<GameFile>(game_settings_file.as_str());
  let verify_level_file = toml::from_str::<LevelFile>(level_file.as_str());
  let verify_sprite_file = toml::from_str::<SpriteFile>(sprite_file.as_str());
  let verify_sprite_types_file = toml::from_str::<SpriteTypesFile>(sprite_types_file.as_str());

  let toml_problems: Vec<Option<String>> = vec![
    find_toml_problems(GAME_SETTING_PATH, verify_game_settings),
    find_toml_problems(LEVEL_FILE_PATH, verify_level_file),
    find_toml_problems(SPRITE_FILE_PATH, verify_sprite_file),
    find_toml_problems(SPRITE_TYPE_FILE_PATH, verify_sprite_types_file),
  ];

  let found_problems = toml_problems
    .iter()
    .filter(|file| file.is_some())
    .map(|file| file.clone().unwrap());
  let should_panic = toml_problems.iter().any(|file| file.is_some());

  for problem in found_problems {
    println!("{}", problem);
  }

  if should_panic {
    panic!("File Verification Failed");
  }
}

fn find_toml_problems<T>(path: &str, toml_result: Result<T, Error>) -> Option<String> {
  if let Err(e) = toml_result {
    Some(format!("File verification failed for {}: {}", path, e))
  } else {
    None
  }
}
