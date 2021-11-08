//! Handles all UI elements for the game.

use bevy::prelude::*;

use diagnostic::{setup_fps_text, update_fps_system};
use title_screen::{setup_title_screen, title_menu_buttons, delete_title_screen, TitleMenuState};

pub use title_screen::LoadTitleScreen;

mod title_screen;
mod diagnostic;

// Spawns a [UiCameraBundle]
fn setup_uicamera(mut commands: Commands) {
  commands.spawn_bundle(UiCameraBundle::default());
}

/// [Plugin] for handling ui elements
pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<TitleMenuState>()
      .add_startup_system(setup_uicamera.system())
      .add_startup_system(setup_fps_text.system())
      .add_system(setup_title_screen.system())
      .add_system(update_fps_system.system())
      .add_system(delete_title_screen.system())
      .add_system(title_menu_buttons.system());
  }
}
