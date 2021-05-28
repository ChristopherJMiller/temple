use std::fs;

use bevy::prelude::*;
use kurinji::{Kurinji, KurinjiPlugin};

fn setup_inputs(mut kurinji: ResMut<Kurinji>) {
  if let Ok(bindings) = fs::read_to_string("assets/inputs/keyboard.ron") {
    kurinji.set_bindings_with_ron(&bindings);
  } else {
    panic!("Unable to load input file!");
  }
}

// TODO: Swap for Kurinji based resource for bringing up menu
fn dev_toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
  let window = windows.get_primary_mut().unwrap();
  if input.just_pressed(KeyCode::Escape) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
  }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_plugin(KurinjiPlugin)
      .add_startup_system(setup_inputs.system())
      .add_system(dev_toggle_cursor.system());
  }
}
