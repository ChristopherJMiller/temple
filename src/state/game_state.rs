use std::collections::HashMap;
use std::fs::{create_dir, read, read_dir, write};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::level::LevelId;
use crate::util::files::from_game_root;

/// Defines the current state of the game, used to provide context to other systems (level saving, interfaces, etc)
#[derive(Clone, Debug, PartialEq)]
pub enum GameMode {
  /// Game is at MainMenu
  MainMenu,
  /// Editor is Active, [GameMode] will not change
  EditMode,
  /// Game is in Overworld
  #[allow(dead_code)]
  Overworld,
  /// Game has entered a level. [LevelId] defines the entrylevel
  InLevel(LevelId)
}

impl Default for GameMode {
  fn default() -> Self {
    Self::MainMenu
  }
}

#[derive(Clone, Debug, Default)]
pub struct TempleState {
  pub game_mode: GameMode,
}

impl TempleState {
  pub fn edit_mode() -> Self {
    TempleState { game_mode: GameMode::EditMode }
  }

  pub fn in_edit_mode(&self) -> bool {
    self.game_mode == GameMode::EditMode
  }
}

/// Describes the clear state of a given visted level.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LevelClearState {
  NotCleared,
  AtCheckpoint(LevelId, f32, f32),
  Cleared,
}

/// Game save file to manage game flags and level clears
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSaveState {
  pub name: String,
  pub level_clears: HashMap<String, LevelClearState>,
}

impl GameSaveState {
  /// Creates a valid string key for use in TOML (numbers can't be used)
  pub fn key(id: LevelId) -> String {
    format!("L{}", id)
  }

  pub fn new(name: String) -> Self {
    Self {
      name,
      level_clears: Default::default(),
    }
  }
}

const SAVES_PATH: &str = "saves";

/// [Res] of loaded save files
pub struct AvaliableSaves(pub HashMap<String, GameSaveState>);

/// [Res] of the selected save file.
#[derive(Clone, Default)]
pub struct ActiveSave(pub Option<GameSaveState>);

impl ActiveSave {
  pub fn get_level_state(&self, key: LevelId) -> Option<&LevelClearState> {
    if let Some(game_saves) = &self.0 {
      game_saves.level_clears.get(&GameSaveState::key(key))
    } else {
      None
    }
  }
}

/// Loads the `saves/` directory and any valid present save files
pub fn bootstrap_and_get_saves() -> HashMap<String, GameSaveState> {
  let saves_dir = from_game_root(SAVES_PATH);
  if !saves_dir.is_dir() {
    create_dir(saves_dir).expect("Unable to setup saves directory");
    return HashMap::default();
  }

  let save_files = read_dir(saves_dir).expect("Unable to read files in saves directory");

  let mut saves = HashMap::default();

  for file in save_files.into_iter() {
    if let Ok(entry) = file {
      if let Ok(contents) = read(&entry.path().into_os_string().to_str().unwrap()) {
        match toml::from_slice::<GameSaveState>(&contents) {
          Ok(save) => {
            saves.insert(save.name.clone(), save);
          },
          Err(err) => {
            warn!(target: "bootstrap_and_get_saves", "Failed to load {:?}: {}", entry.path(), err.to_string());
          },
        }
      }
    }
  }

  saves
}

/// Writes a save file
pub fn write_save(save: &GameSaveState) {
  let saves_dir = from_game_root(SAVES_PATH);
  let mut save_path = saves_dir.join(save.name.clone());
  let contents = toml::to_string_pretty(&save.clone()).unwrap();
  save_path.set_extension("toml");
  if write(save_path, contents).is_err() {
    info!(target: "write_saves", "Was unable to save the game!");
  }
}

/// Writes the [HashMap] of save files used by [AvaliableSaves]
pub fn write_saves(saves: &HashMap<String, GameSaveState>) {
  for (_, save) in saves {
    write_save(save);
  }
}
