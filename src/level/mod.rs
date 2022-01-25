//! Level config management and level loading.

use bevy::prelude::*;
use load::{apply_save_on_load, load_level, prepare_level, transition_level, unload_level};
use verify::verify_level_files;

use self::next::auto_next_level;

pub mod config;
pub mod load;
pub mod next;
pub mod save;
pub mod util;
pub mod verify;

pub type LevelId = u32;

/// [Plugin] for level management systems
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(verify_level_files.system())
      .add_system(prepare_level.system())
      .add_system(load_level.system())
      .add_system(unload_level.system())
      .add_system(apply_save_on_load.system())
      .add_system(transition_level.system())
      .add_system(auto_next_level.system());
  }
}
