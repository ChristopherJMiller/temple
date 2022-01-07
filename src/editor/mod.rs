use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::EditorCameraPlugin;

use self::ui::EditorUiPlugin;

pub mod camera;
pub mod state;
pub mod ui;

/// Flags that the engine is in editor mode
pub struct EditorMode;

/// [PluginGroup] for the editor.
pub struct EditorPlugins;

impl PluginGroup for EditorPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group.add(EditorCameraPlugin).add(EditorUiPlugin);
  }
}
