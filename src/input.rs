//! Input config management and associated systems.

use std::fs;

use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin};

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

/// Loads [Kurinji] input config files
fn setup_inputs(mut kurinji: ResMut<Kurinji>) {
  if let Ok(bindings) = fs::read_to_string("assets/inputs/keyboard.ron") {
    kurinji.set_bindings_with_ron(&bindings);
  } else {
    panic!("Unable to load input file!");
  }
}

/// Temporary component to handle locking and unlocking the cursor within the window.
/// To be replaced with menu-based cursor toggling
struct DevToggleCursor(pub bool);

/// System for toggling the cursor via [DevToggleCursor].
fn dev_toggle_cursor(input: Res<Kurinji>, mut cursor_flag: ResMut<DevToggleCursor>, mut windows: ResMut<Windows>) {
  if input.is_action_active(MENU) && !cursor_flag.0 {
    cursor_flag.0 = true;
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
  } else if !input.is_action_active(MENU) && cursor_flag.0 {
    cursor_flag.0 = false;
  }
}

/// [Plugin] for input systems.
pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      // Kurinji Input Boostrapping
      .add_plugin(KurinjiPlugin)
      .add_startup_system(setup_inputs.system())

      // Dev Systems
      .insert_resource(DevToggleCursor(false))
      .add_system(dev_toggle_cursor.system());
  }
}
