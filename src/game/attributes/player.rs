//! Represents the player. `player()`

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::{Attribute, Checkpoint, Transition, Deadly};
use crate::game::collision::PlayerContacted;
use crate::game::collision_groups::*;
use crate::game::sfx::{AudioChannels, SfxHandles};
use crate::level::LevelId;
use crate::level::load::{LevelLoadComplete, LoadLevel, TransitionLevel};
use crate::state::game_state::{write_save, ActiveSave, GameSaveState, LevelSaveState, TempleState, GameMode, CheckpointState};

/// Active Player State
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
      height_adjust: 0.25,
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
      },
      damping: RigidBodyDamping {
        linear_damping: 1.5,
        ..Default::default()
      },
      ..Default::default()
    };
    let collider = ColliderBundle {
      position: Vec2::ZERO.into(),
      material: ColliderMaterial {
        friction: 0.0,
        ..Default::default()
      },
      shape: ColliderShape::ball(0.5),
      flags: ColliderFlags {
        collision_groups: PLAYER_GROUP,
        solver_groups: PLAYER_GROUP,
        ..Default::default()
      },
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
  deadly_contacted: Query<Entity, (With<Deadly>, With<PlayerContacted>)>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  mut player: Query<(&mut RigidBodyPosition, &Player)>,
) {
  if let Ok((mut pos, player)) = player.single_mut() {
    deadly_contacted.for_each(|ent| {
      let level_id = loaded_level.single().unwrap().0;
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
  checkpoint_reached: Query<(Entity, &Checkpoint), With<PlayerContacted>>,
  mut player: Query<&mut Player>,
  audio: Res<Audio>,
  sfx_handles: Res<SfxHandles>,
  channels: Res<AudioChannels>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  temple_state: Res<TempleState>,
  mut active_save: ResMut<ActiveSave>,
) {
  if let Ok(mut player) = player.single_mut() {
    checkpoint_reached.for_each(|(ent, checkpoint)| {
      if player.respawn_pos != checkpoint.0 {
        audio.play_in_channel(sfx_handles.checkpoint.clone(), &channels.sfx.0);
        if let Some(save) = &mut active_save.0 {
          if let Ok(level) = loaded_level.single() {
            if let GameMode::InLevel(level_entry) = temple_state.game_mode {
              player.respawn_level = level.0;
              player.respawn_pos = checkpoint.0;
              let key = GameSaveState::key(level_entry);
              if let Some(save) = save.level_clears.get_mut(&key) {
                save.set_checkpoint(CheckpointState::AtCheckpoint(level.0, checkpoint.0.x, checkpoint.0.y))
              } else {
                save.level_clears.insert(
                  GameSaveState::key(level_entry),
                  LevelSaveState::new_with_checkpoint(CheckpointState::AtCheckpoint(level.0, checkpoint.0.x, checkpoint.0.y)),
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

pub fn on_transition_system(
  mut commands: Commands,
  transition_activated: Query<(Entity, &Transition), With<PlayerContacted>>
) {
  for (entity, trans) in transition_activated.iter() {
    commands.spawn().insert(TransitionLevel(trans.0));
    commands.entity(entity).remove::<PlayerContacted>();
  }
}
