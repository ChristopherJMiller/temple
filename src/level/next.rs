use bevy::prelude::*;

use super::load::{LoadLevel, TransitionLevel};
use crate::game::credits::PlayCredits;
use crate::level::load::UnloadLevel;
use crate::state::game_state::{GameMode, TempleState};
use crate::util::settings::{GameFile, LevelTransistionType};

/// Command to determine next level to play, and transition to that level. Used
/// after reaching a [Goal] when in `NoOverworld` mode.
#[derive(Component)]
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
            if let Ok(ent) = loaded_level.get_single() {
              commands.entity(ent).insert(UnloadLevel);
              commands.spawn().insert(PlayCredits);
            }
          }
        }
      }
    }
  });
}

#[cfg(test)]
mod tests {
  use std::fs;

  use crate::level::next::*;
  use crate::level::LevelId;
  use crate::state::game_state::*;
  use crate::util::files::*;
  use crate::util::settings::*;

  fn setup_world(current_level: LevelId) -> (World, SystemStage) {
    fs::create_dir_all(from_game_root(LEVEL_DIR_PATH)).unwrap();

    let mut world = World::default();

    // Set up Resources
    let game_file = GameFile {
      title: "".to_string(),
      authors: vec![],
      level_transistion: LevelTransistionType::NoOverworld,
      level_order: Some(vec![0, 1]),
      credits: "".to_string(),
      credit_music: "".to_string(),
    };

    let temple_state = TempleState {
      game_mode: GameMode::InLevel(current_level),
    };

    world.insert_resource(game_file);
    world.insert_resource(temple_state);

    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(auto_next_level);

    (world, update_stage)
  }

  #[test]
  fn test_auto_next_level() {
    let (mut world, mut update_stage) = setup_world(0);

    // Spawn Commands
    world.spawn().insert(LoadLevel(0));
    world.spawn().insert(NextLevel);

    // Run
    update_stage.run(&mut world);

    // Check for transistionlevel command to level 1
    assert_eq!(world.query::<&TransitionLevel>().iter(&world).next().unwrap().0, 1);
  }

  #[test]
  fn test_auto_next_level_credits() {
    let (mut world, mut update_stage) = setup_world(1);

    // Spawn Commands (note is final level of level order)
    world.spawn().insert(LoadLevel(1));
    world.spawn().insert(NextLevel);

    // Run
    update_stage.run(&mut world);

    assert_eq!(world.query::<&PlayCredits>().iter(&world).len(), 1);
  }
}
