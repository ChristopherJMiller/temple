use bevy::prelude::*;
use kurinji::Kurinji;
use crate::input::{UP, DOWN, LEFT, RIGHT};

fn handle_player_movement(
  input: Res<Kurinji>
) {
  if input.is_action_active(UP) {
    info!(target: "handle_player_movement", "Up!");
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(handle_player_movement.system());
  }
}
