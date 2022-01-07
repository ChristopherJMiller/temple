//! Level config management and level loading.

use bevy::prelude::*;
use load::{apply_save_on_load, load_level, unload_level};

use self::load::prepare_level;
use self::verify::verify_level_files;

pub mod config;
pub mod load;
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
      .add_system(apply_save_on_load.system());
  }
}
