use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::EditorCameraPlugin;

pub mod camera;

/// Flags that the engine is in editor mode
pub struct EditorMode;


/// [PluginGroup] for the editor.
pub struct EditorPlugins;

impl PluginGroup for EditorPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group.add(EditorCameraPlugin);
  }
}
