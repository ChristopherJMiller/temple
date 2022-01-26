use bevy::prelude::*;

use super::load::{TransitionLevel, LoadLevel};
use crate::game::credits::PlayCredits;
use crate::level::load::UnloadLevel;
use crate::state::game_state::{GameMode, TempleState};
use crate::util::settings::{GameFile, LevelTransistionType};

/// Command to determine next level to play, and transition to that level. Used
/// after reaching a [Goal] when in `NoOverworld` mode.
pub struct NextLevel;

pub fn auto_next_level(
  mut commands: Commands,
  loaded_level: Query<Entity, With<LoadLevel>>,
  next_level: Query<Entity, With<NextLevel>>,
  mut temple_state: ResMut<TempleState>,
  game_file: Res<GameFile>,
) {
  next_level.for_each(|ent| {
    commands.entity(ent).despawn();
    if let GameMode::InLevel(level) = temple_state.game_mode {
      if game_file.level_transistion == LevelTransistionType::NoOverworld {
        let order = game_file.level_order.clone().unwrap();
        let mut iter = order.iter();
        if iter.find(|&&x| x == level).is_some() {
          if let Some(next_level) = iter.next() {
            temple_state.game_mode = GameMode::InLevel(*next_level);
            commands.spawn().insert(TransitionLevel(*next_level));
          } else {
            info!(target: "auto_next_level", "End of Game!");
            if let Ok(ent) = loaded_level.single() {
              commands.entity(ent).insert(UnloadLevel);
              commands.spawn().insert(PlayCredits);
            }
          }
        }
      }
    }
  });
}