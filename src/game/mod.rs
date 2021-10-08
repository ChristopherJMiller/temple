//! Game mechanisms and sprite attribute definitions.

use attributes::AttributePlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use collision::CollisionPlugin;

pub mod attributes;
pub mod camera;
pub mod collision_groups;
pub mod player;
pub mod collision;

/// [PluginGroup] for game modules.
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(CameraPlugin)
      .add(PlayerPlugin)
      .add(AttributePlugin)
      .add(CollisionPlugin);
  }
}
