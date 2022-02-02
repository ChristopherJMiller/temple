//! Represents the player. `player()`

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::{Attribute, Checkpoint, Deadly, Give, Goal, Transition};
use crate::game::collision::{ContactQuery, ContactTagQuery, PlayerContacted};
use crate::game::collision_groups::*;
use crate::game::sfx::{AudioChannels, SfxHandles};
use crate::level::load::{LevelLoadComplete, LoadLevel, TransitionLevel};
use crate::level::next::NextLevel;
use crate::level::LevelId;
use crate::state::game_state::{write_save, ActiveSave, GameMode, GameSaveState, LevelSaveState, TempleState};
use crate::util::settings::{GameFile, LevelTransistionType};

/// Active Player State
#[derive(Component)]
pub struct Player {
  pub height_adjust: f32,
  pub grounded: bool,
  pub jump_boost_time: f32,
  pub jump_in_progress: bool,
  pub outside_ground_bounds: bool,
  pub on_moving_entity: Option<Entity>,
  pub respawn_level: LevelId,
  pub respawn_pos: Vec2,
}

impl Player {
  pub const JUMP_BOOST_TIME: f32 = 0.35;
  pub const NORMAL_FALL_SPEED: f32 = 2.25;
  pub const SLOW_FALL_SPEED: f32 = 1.25;

  pub fn new(respawn_level: LevelId, respawn_pos: Vec2) -> Self {
    Self {
      height_adjust: 2.0,
      grounded: true,
      jump_boost_time: Self::JUMP_BOOST_TIME,
      jump_in_progress: false,
      outside_ground_bounds: false,
      on_moving_entity: None,
      respawn_level,
      respawn_pos,
    }
  }
}

impl Attribute for Player {
  const KEY: &'static str = "player";

  fn build(commands: &mut Commands, target: Entity, level: LevelId, position: Vec2, _: Vec<ParseArgumentItem>) {
    let rigid_body = RigidBodyBundle {
      position: position.into(),
      mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
      forces: RigidBodyForces {
        gravity_scale: Self::NORMAL_FALL_SPEED,
        ..Default::default()
      }
      .into(),
      damping: RigidBodyDamping {
        linear_damping: 1.5,
        ..Default::default()
      }
      .into(),
      ..Default::default()
    };
    let collider = ColliderBundle {
      position: Vec2::ZERO.into(),
      material: ColliderMaterial {
        friction: 0.0,
        ..Default::default()
      }
      .into(),
      shape: ColliderShape::ball(0.5).into(),
      flags: ColliderFlags {
        collision_groups: PLAYER_GROUP,
        solver_groups: PLAYER_GROUP,
        ..Default::default()
      }
      .into(),
      ..Default::default()
    };

    commands
      .entity(target)
      .insert(Player::new(level, position.into()))
      .insert_bundle(rigid_body)
      .insert_bundle(collider)
      .insert(RigidBodyPositionSync::Interpolated { prev_pos: None });
  }
}

/// Consumes [PlayerDied] tags and respawns the player.
pub fn on_death_system(
  mut commands: Commands,
  deadly_contacted: ContactTagQuery<Deadly>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  mut player: Query<(&mut RigidBodyPositionComponent, &Player)>,
) {
  if let Ok((mut pos, player)) = player.get_single_mut() {
    deadly_contacted.for_each(|ent| {
      let level_id = loaded_level.get_single().unwrap().0;
      if player.respawn_level != level_id {
        commands.spawn().insert(TransitionLevel(player.respawn_level));
      } else {
        pos.position.translation = player.respawn_pos.into();
      }

      commands.entity(ent).remove::<PlayerContacted>();
    });
  }
}

/// Consumes [PlayerContacted] tags and sets the new player respawn
/// point.
pub fn on_checkpoint_system(
  mut commands: Commands,
  checkpoint_reached: ContactQuery<Checkpoint>,
  mut player: Query<&mut Player>,
  audio: Res<Audio>,
  sfx_handles: Res<SfxHandles>,
  channels: Res<AudioChannels>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  temple_state: Res<TempleState>,
  mut active_save: ResMut<ActiveSave>,
) {
  if let Ok(mut player) = player.get_single_mut() {
    checkpoint_reached.for_each(|(ent, checkpoint)| {
      if player.respawn_pos != checkpoint.0 {
        audio.play_in_channel(sfx_handles.checkpoint.clone(), &channels.sfx.0);
        if let Some(save) = &mut active_save.0 {
          if let Ok(level) = loaded_level.get_single() {
            if let GameMode::InLevel(level_entry) = temple_state.game_mode {
              player.respawn_level = level.0;
              player.respawn_pos = checkpoint.0;
              let key = GameSaveState::key(level_entry);
              if let Some(save) = save.level_clears.get_mut(&key) {
                save.set_checkpoint((level.0, checkpoint.0.x, checkpoint.0.y))
              } else {
                save.level_clears.insert(
                  GameSaveState::key(level_entry),
                  LevelSaveState::new_with_checkpoint((level.0, checkpoint.0.x, checkpoint.0.y)),
                );
              }

              write_save(save);
            }
          }
        }
      }

      commands.entity(ent).remove::<PlayerContacted>();
    });
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

pub fn on_transition_system(mut commands: Commands, transition_activated: ContactQuery<Transition>) {
  for (entity, trans) in transition_activated.iter() {
    commands.spawn().insert(TransitionLevel(trans.0));
    commands.entity(entity).remove::<PlayerContacted>();
  }
}

pub fn on_give_system(mut commands: Commands, player: Query<Entity, With<Player>>, goal: ContactQuery<Give>) {
  if let Ok(player) = player.get_single() {
    goal.for_each(|(ent, give)| {
      commands.entity(player).insert(give.build_component());
      commands.entity(ent).despawn();
    });
  }
}
