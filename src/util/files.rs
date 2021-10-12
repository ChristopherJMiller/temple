//! Handles manifest validation
//!
//! # About
//! Given temple's reliance on configurability via manifests,
//! steps should be taken to ensure all config files are in a valid
//! state before game start.
//!
//! This is currently handled by the entrypoint [verify_files], which:
//! - Loads all files from their default locations.
//! - Parses them from their TOML string.
//! - Reports any found issues.

use std::fs;

use toml::de::Error;

use crate::level::config::LevelFile;
use crate::sprite::{SpriteFile, SpriteTypesFile};
use crate::util::settings::{GameFile, LevelTransistionType};

/// `game.toml` location
pub const GAME_SETTING_PATH: &str = "assets/game.toml";

/// `levels.toml` location
pub const LEVEL_FILE_PATH: &str = "assets/levels.toml";

/// `sprites.toml` location
pub const SPRITE_FILE_PATH: &str = "assets/sprites/sprites.toml";

/// `types.toml` location
pub const SPRITE_TYPE_FILE_PATH: &str = "assets/sprites/types.toml";

/// Verifies all game config files are found and valid.
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
  let game_settings_copy = verify_game_settings.clone();

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

  // Verify Game Settings Attributes
  let game_settings = game_settings_copy.unwrap();
  if game_settings.level_transistion == LevelTransistionType::NoOverworld && game_settings.level_order.is_none() {
    panic!("game.toml: NoOverworld supplied for level_transistion, but no level_order was provided!");
  }
}

/// Consumes the TOML result and reports any errors found.
fn find_toml_problems<T>(path: &str, toml_result: Result<T, Error>) -> Option<String> {
  if let Err(e) = toml_result {
    Some(format!("File verification failed for {}: {}", path, e))
  } else {
    None
  }
}
