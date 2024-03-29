//! Completes an exit for the active level. `goal(exit number)`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::game::collision::{ContactQuery, ContactSubscription, PlayerContacted};
use crate::game::collision_groups::*;
use crate::level::next::NextLevel;
use crate::level::LevelId;
use crate::state::game_state::{write_save, ActiveSave, GameMode, GameSaveState, TempleState};
use crate::util::settings::{GameFile, LevelTransistionType};

#[derive(Component)]
pub struct Goal(pub usize);

fn parse_params(params: Vec<ParseArgumentItem>) -> usize {
  let arg = params
    .get(0)
    .unwrap_or_else(|| panic!("Goal attribute created with no exit number parameter!"));
  if let ParseArgumentItem::Number(exit) = arg {
    exit
      .clone()
      .try_into()
      .unwrap_or_else(|_| panic!("Invalid number provided for exit number!"))
  } else {
    panic!("Non-number parameter provided to goal attribute!");
  }
}

impl Attribute for Goal {
  const KEY: &'static str = "goal";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, params: Vec<ParseArgumentItem>) {
    let exit_number = parse_params(params);

    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(0.5, 0.5).into(),
      material: ColliderMaterialComponent::default(),
      flags: ColliderFlags {
        collision_groups: DETECTS_PLAYER_GROUP,
        solver_groups: NONE_GROUP,
        active_events: ActiveEvents::CONTACT_EVENTS,
        ..Default::default()
      }
      .into(),
      ..Default::default()
    };

    commands
      .entity(target)
      .insert(Goal(exit_number))
      .insert(ContactSubscription)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}

pub fn on_goal_system(
  mut commands: Commands,
  goal_reached: ContactQuery<Goal>,
  temple_state: Res<TempleState>,
  game_file: Res<GameFile>,
  mut active_save: ResMut<ActiveSave>,
) {
  goal_reached.for_each(|(ent, goal)| {
    // Get active save file
    if let Some(save) = &mut active_save.0 {
      // Get what level player is currently in
      if let GameMode::InLevel(level) = temple_state.game_mode {
        // Save exit clear
        if let Some(level) = save.level_clears.get_mut(&GameSaveState::key(level)) {
          level.clear_exit(goal.0);
          write_save(save);
        }
      }
    } else {
      warn!(target: "on_goal_system", "No active save to clear level on. Ignoring...");
    }

    if game_file.level_transistion == LevelTransistionType::NoOverworld {
      commands.spawn().insert(NextLevel);
    }

    commands.entity(ent).remove::<PlayerContacted>();
  });
}
