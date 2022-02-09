//! Level config management and level loading.

use bevy::prelude::*;
use load::{apply_save_on_load, load_level, prepare_level, transition_level, unload_level};
use verify::verify_level_files;

use self::load::wait_until_unloaded;
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
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(verify_level_files)
      .add_system(wait_until_unloaded)
      .add_system(prepare_level)
      .add_system(load_level)
      .add_system(unload_level)
      .add_system(apply_save_on_load)
      .add_system(transition_level)
      .add_system(auto_next_level);
  }
}
