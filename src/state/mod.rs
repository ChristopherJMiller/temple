use bevy::prelude::*;
use game_state::{bootstrap_and_get_saves, ActiveSave, AvaliableSaves};

use self::settings::Settings;

pub mod game_state;
pub mod settings;

/// [Plugin] for manging persistent game states (settings, saves, etc.)
pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut App) {
    let saves = bootstrap_and_get_saves();

    app
      .insert_resource(AvaliableSaves(saves))
      .init_resource::<ActiveSave>()
      .init_resource::<Settings>();
  }
}
