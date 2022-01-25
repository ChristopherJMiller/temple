use std::collections::HashMap;
use std::fs::{create_dir, read, read_dir, write};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::level::LevelId;
use crate::util::files::from_game_root;

/// Defines the current state of the game, used to provide context to other
/// systems (level saving, interfaces, etc)
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
  InLevel(LevelId),
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
    TempleState {
      game_mode: GameMode::EditMode,
    }
  }

  pub fn in_edit_mode(&self) -> bool {
    self.game_mode == GameMode::EditMode
  }
}

pub type CheckpointState = (LevelId, f32, f32);

/// Describes the state of a level save, including exits completed and current
/// checkpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelSaveState {
  exits_cleared: Vec<bool>,
  checkpoint_state: Option<CheckpointState>,
}

impl LevelSaveState {
  pub fn new_with_checkpoint(state: CheckpointState) -> Self {
    Self {
      exits_cleared: Vec::new(),
      checkpoint_state: Some(state),
    }
  }

  /// Clears an exit on the level
  pub fn clear_exit(&mut self, exit_num: usize) {
    if exit_num > self.exits_cleared.len() {
      self.exits_cleared.resize(exit_num, false);
    }
    self.exits_cleared.insert(exit_num, true);
    self.checkpoint_state = None;
  }

  #[allow(dead_code)]
  pub fn exit_cleared(&self, exit_num: usize) -> bool {
    if let Some(exit_state) = self.exits_cleared.get(exit_num) {
      *exit_state
    } else {
      false
    }
  }

  /// Sets the checkpoint
  pub fn set_checkpoint(&mut self, state: CheckpointState) {
    self.checkpoint_state = Some(state);
  }

  /// Returns the checkpoint data, if any
  pub fn checkpoint(&self) -> &Option<CheckpointState> {
    &self.checkpoint_state
  }

  /// Returns true if any exit is cleared on this level. Used for `NoOverworld` level order calculations
  pub fn an_exit_cleared(&self) -> bool {
    self.exits_cleared.iter().find(|&&x| x).is_some()
  }
}

impl Default for LevelSaveState {
  fn default() -> Self {
    Self {
      exits_cleared: Vec::new(),
      checkpoint_state: None,
    }
  }
}

/// Game save file to manage game flags and level clears
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSaveState {
  pub name: String,
  pub level_clears: HashMap<String, LevelSaveState>,
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
  pub fn get_level_state(&self, key: LevelId) -> Option<&LevelSaveState> {
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
  let contents = toml::to_string_pretty(&save.clone())
    .unwrap_or_else(|err| panic!("Error occured when writing save: {}", err.to_string()));
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

#[cfg(test)]
mod tests {
  use crate::state::game_state::*;

  #[test]
  fn test_game_save() {
    let mut save = LevelSaveState::default();
    assert_eq!(false, save.exit_cleared(0));
    save.clear_exit(1);
    assert_eq!(false, save.exit_cleared(0));
    assert_eq!(true, save.exit_cleared(1));
    assert_eq!(false, save.exit_cleared(2));
    save.clear_exit(5);
    assert_eq!(true, save.exit_cleared(1));
    assert_eq!(true, save.exit_cleared(5));
  }
}
