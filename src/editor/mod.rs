use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::EditorCameraPlugin;

use self::ui::EditorUiPlugin;

pub mod camera;
pub mod ui;

/// [PluginGroup] for the editor.
pub struct EditorPlugins;

impl PluginGroup for EditorPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group.add(EditorCameraPlugin).add(EditorUiPlugin);
  }
}
