use std::collections::HashMap;

use bevy::prelude::*;
use config::{load_level_files, Level, LevelFileVersion};
use load::{configure_rapier, load_level, unload_level};

use crate::sprite::SpritePluginSteps;

pub mod config;
pub mod load;

pub type LevelId = u32;
pub type LevelMap = HashMap<LevelId, Level>;

// Level Plugin

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource::<LevelFileVersion>(LevelFileVersion(1))
      .init_resource::<LevelMap>()
      .add_startup_system(configure_rapier.system())
      .add_startup_system(load_level_files.system().after(SpritePluginSteps::LoadSprites))
      .add_system(load_level.system())
      .add_system(unload_level.system());
  }
}
