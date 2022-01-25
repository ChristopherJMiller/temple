use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::EditorCameraPlugin;

use self::border::EditorBorderPlugin;
use self::sprite::EditorSpritePlugin;
use self::ui::EditorUiPlugin;

pub mod border;
pub mod camera;
pub mod sprite;
pub mod ui;
pub mod util;

/// [PluginGroup] for the editor.
pub struct EditorPlugins;

impl PluginGroup for EditorPlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(EditorCameraPlugin)
      .add(EditorUiPlugin)
      .add(EditorSpritePlugin)
      .add(EditorBorderPlugin);
  }
}
