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

use std::env::current_exe;
use std::fs;
use std::path::{Path, PathBuf};

use const_format::concatcp;
use toml::de::Error;

use crate::level::config::LevelManifest;
use crate::util::settings::{GameFile, LevelTransistionType};

/// Asset Root Location
const ASSET_PATH: &str = "assets/";

/// `game.toml` location
pub const GAME_SETTING_PATH: &str = concatcp!(ASSET_PATH, "game.toml");

/// `/levels/` location
pub const LEVEL_DIR_PATH: &str = concatcp!(ASSET_PATH, "levels/");

/// `/levelmaps/` location
pub const LEVEL_MAP_DIR_PATH: &str = concatcp!(ASSET_PATH, "levelmaps/");

/// `/textures/` location
pub const TEXTURE_DIR_PATH: &str = concatcp!(ASSET_PATH, "textures/");
pub const SPRITE_TEXTURE_DIR_PATH: &str = concatcp!(TEXTURE_DIR_PATH, "sprites/");

/// `/audio/music` location
pub const MUSIC_DIR_PATH: &str = concatcp!(ASSET_PATH, "audio/music/");

pub fn from_game_root<T: AsRef<Path>>(path: T) -> PathBuf {
  let mut base = current_exe().unwrap();
  base.pop();
  if cfg!(debug_assertions) || cfg!(feature = "devrootpath") {
    base.join("../..").join(path)
  } else {
    base.join(path)
  }
}

/// Verifies all game config files are found and valid.
pub fn verify_files() {
  let game_settings_file = fs::read_to_string(from_game_root(GAME_SETTING_PATH))
    .unwrap_or_else(|_| panic!("Failed to open file {:?}", from_game_root(GAME_SETTING_PATH)));
  let level_dir = fs::read_dir(from_game_root(LEVEL_DIR_PATH))
    .unwrap_or_else(|_| panic!("Failed to find directory {}", LEVEL_DIR_PATH));

  let verify_game_settings = toml::from_str::<GameFile>(game_settings_file.as_str());

  let mut toml_problems: Vec<Option<String>> =
    vec![find_toml_problems(GAME_SETTING_PATH, verify_game_settings.clone())];

  // Verify level manifests
  for entry in level_dir {
    if let Ok(entry) = entry {
      if !entry.path().is_dir() {
        let level =
          fs::read_to_string(entry.path()).unwrap_or_else(|_| panic!("Failed to open file {:?}", entry.path()));

        toml_problems.push(find_toml_problems(
          entry.path().to_str().unwrap(),
          toml::from_str::<LevelManifest>(&level),
        ));
      }
    }
  }

  let game_settings_copy = verify_game_settings.clone();

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
