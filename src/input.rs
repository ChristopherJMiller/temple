use std::fs;

use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin};

pub const UP: &str = "UP";
pub const RIGHT: &str = "RIGHT";
pub const LEFT: &str = "LEFT";
pub const DOWN: &str = "DOWN";
pub const JUMP: &str = "JUMP";
pub const MENU: &str = "MENU";

fn setup_inputs(mut kurinji: ResMut<Kurinji>) {
  if let Ok(bindings) = fs::read_to_string("assets/inputs/keyboard.ron") {
    kurinji.set_bindings_with_ron(&bindings);
  } else {
    panic!("Unable to load input file!");
  }
}

struct DevToggleCursor(pub bool);

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

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource(DevToggleCursor(false))
      .add_plugin(KurinjiPlugin)
      .add_startup_system(setup_inputs.system())
      .add_system(dev_toggle_cursor.system());
  }
}
