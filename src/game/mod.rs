use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::CameraPlugin;

pub mod attributes;
pub mod camera;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group.add(CameraPlugin);
  }
}
