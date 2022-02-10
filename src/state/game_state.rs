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

  pub fn in_game(&self) -> bool {
    if let GameMode::InLevel(_) = self.game_mode {
      true
    } else {
      self.game_mode == GameMode::Overworld
    }
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

  /// Returns true if any exit is cleared on this level. Used for `NoOverworld`
  /// level order calculations
  pub fn an_exit_cleared(&self) -> bool {
    self.exits_cleared.iter().find(|&&x| x).is_some()
  }

  /// Returns the list of exits cleared
  pub fn exits_cleared(&self) -> Vec<bool> {
    self.exits_cleared.clone()
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
  #[serde(skip_serializing_if = "HashMap::is_empty", default)]
  pub level_clears: HashMap<String, LevelSaveState>,
}

impl GameSaveState {
  /// Creates a valid string key for use in TOML (numbers can't be used)
  pub fn key(id: LevelId) -> String {
    format!("L{}", id)
  }

  pub fn new<S: ToString>(name: S) -> Self {
    Self {
      name: name.to_string(),
      level_clears: Default::default(),
    }
  }

  pub fn num_cleared_exits(&self) -> usize {
    self.level_clears.values().into_iter().fold(0, |acc, x| acc + x.exits_cleared().iter().filter(|&&x| x).count())
  }
}

#[cfg(not(test))]
const SAVES_PATH: &str = "saves";
#[cfg(test)]
const SAVES_PATH: &str = "test/saves";

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
    assert!(!save.exit_cleared(0));
    save.clear_exit(1);
    assert!(!save.exit_cleared(0));
    assert!(save.exit_cleared(1));
    assert!(!save.exit_cleared(2));
    save.clear_exit(5);
    assert!(save.exit_cleared(1));
    assert!(save.exit_cleared(5));
    assert!(save.an_exit_cleared());

    let mut save2 = LevelSaveState::new_with_checkpoint((0, 5.0, 10.0));
    assert_eq!(save2.checkpoint().unwrap(), (0, 5.0, 10.0));
    save2.set_checkpoint((0, 10.0, 20.0));
    assert_eq!(save2.checkpoint().unwrap(), (0, 10.0, 20.0));
  }

  #[test]
  fn test_save_bootstrapping() {
    let _ = bootstrap_and_get_saves();
    write_saves(&HashMap::from([
      ("test1".to_string(), GameSaveState::new("test1")),
      ("test2".to_string(), GameSaveState::new("test2")),
    ]));
    let new_saves = bootstrap_and_get_saves();
    assert!(new_saves.contains_key("test1"));
    assert!(new_saves.contains_key("test2"));
  }

  #[test]
  fn test_gamesave_key() {
    assert_eq!("L0".to_string(), GameSaveState::key(0));
  }

  #[test]
  fn test_temple_state() {
    let main_menu = TempleState::default();
    assert_eq!(GameMode::MainMenu, main_menu.game_mode);
    let mut temple_state = TempleState::edit_mode();
    assert!(temple_state.in_edit_mode());
    assert!(!temple_state.in_game());
    temple_state.game_mode = GameMode::InLevel(0);
    assert!(temple_state.in_game());
    temple_state.game_mode = GameMode::Overworld;
    assert!(temple_state.in_game());
  }

  #[test]
  fn test_game_save_state() {
    let mut game = GameSaveState::new("test");
    game.level_clears.insert(GameSaveState::key(0), LevelSaveState::new_with_checkpoint((0, 5.0, 10.0)));
    game.level_clears.get_mut(&GameSaveState::key(0)).unwrap().clear_exit(0);
    game.level_clears.get_mut(&GameSaveState::key(0)).unwrap().clear_exit(5);
    assert_eq!(2, game.num_cleared_exits());
  }
}
