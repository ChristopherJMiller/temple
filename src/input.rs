//! Input config management and associated systems.

use std::{fs, collections::VecDeque};

use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin};

use crate::util::files::from_game_root;

/// Input key for up direction input
pub const UP: &str = "UP";
/// Input key for right direction input
pub const RIGHT: &str = "RIGHT";
/// Input key for left direction input
pub const LEFT: &str = "LEFT";
/// Input key for down direction input
pub const DOWN: &str = "DOWN";
/// Input key for jump action
pub const JUMP: &str = "JUMP";
/// Input key for menu action
pub const MENU: &str = "MENU";
/// Input key for edit mode zoom out
pub const EDIT_ZOOM_OUT: &str = "EDIT_ZOOM_OUT";
/// Input key for edit mode zoom in
pub const EDIT_ZOOM_IN: &str = "EDIT_ZOOM_IN";
/// Input key for "left click" select
pub const SELECT: &str = "SELECT";
/// Input key for "right click" return
pub const RETURN: &str = "RETURN";
/// Input key for toggling sprite borders in edit mode
pub const EDIT_TOGGLE_BORDER: &str = "EDIT_TOGGLE_BORDER";
/// Input key for moving dash controls left (usually used on mouse)
pub const DASH_LEFT: &str = "DASH_LEFT";
/// Input key for moving dash controls right (usually used on mouse)
pub const DASH_RIGHT: &str = "DASH_RIGHT";
/// Input key for moving dash controls up (usually used on mouse)
pub const DASH_UP: &str = "DASH_UP";
/// Input key for moving dash controls down (usually used on mouse)
pub const DASH_DOWN: &str = "DASH_DOWN";

/// Loads [Kurinji] input config files
fn setup_inputs(mut kurinji: ResMut<Kurinji>) {
  if let Ok(bindings) = fs::read_to_string(from_game_root("assets/inputs/keyboard.ron")) {
    kurinji.set_bindings_with_ron(&bindings);
  } else {
    panic!("Unable to load input file!");
  }
}

/// Toggle cursor command queue
#[derive(Default)]
pub struct CursorCommands(VecDeque<bool>);

impl CursorCommands {
  pub fn lock_cursor(&mut self) {
    self.0.push_back(true);
  }

  pub fn unlock_cursor(&mut self) {
    self.0.push_back(false);
  }

  pub fn pop(&mut self) -> Option<bool> {
    self.0.pop_front()
  }
}

/// System for toggling the cursor via [CursorCommands].
fn handle_toggle_cursor(mut queue: ResMut<CursorCommands>, mut windows: ResMut<Windows>) {
  if let Some(lock) = queue.pop() {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(lock);
    window.set_cursor_visibility(!lock);
  }
}

/// [Plugin] for input systems.
pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app
      // Kurinji Input Boostrapping
      .add_plugin(KurinjiPlugin)
      .add_startup_system(setup_inputs)

      // Cursor System
      .init_resource::<CursorCommands>()
      .add_system(handle_toggle_cursor);
  }
}
