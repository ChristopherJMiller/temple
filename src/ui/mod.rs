//! Handles all UI elements for the game.

use bevy::prelude::*;
use diagnostic::{setup_fps_text, update_fps_system};
use font::setup_egui_font;
pub use title_screen::LoadTitleScreen;
use title_screen::{delete_title_screen, setup_title_screen, title_menu_buttons, TitleMenuState};

use self::overlay::{handle_overlay, OverlayCommands};
use self::pause::pause_menu_buttons;

mod diagnostic;
mod font;
pub mod overlay;
mod pause;
pub mod settings;
mod title_screen;

// Spawns a [UiCameraBundle]
fn setup_uicamera(mut commands: Commands) {
  commands.spawn_bundle(UiCameraBundle::default());
}

/// [Plugin] for handling ui elements
pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<TitleMenuState>()
      .init_resource::<OverlayCommands>()
      .add_startup_system(setup_egui_font)
      .add_startup_system(setup_uicamera)
      .add_startup_system(setup_fps_text)
      .add_system(setup_title_screen)
      .add_system(update_fps_system)
      .add_system(delete_title_screen)
      .add_system(title_menu_buttons)
      .add_system(pause_menu_buttons)
      .add_system(handle_overlay);
  }
}
