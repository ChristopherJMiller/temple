//! Represents the player. `player()`

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;

use super::{Attribute, Checkpoint, PlayerReachedCheckpoint};
use crate::game::collision_groups::*;
use crate::game::sfx::{AudioChannels, SfxHandles};
use crate::level::load::{LevelLoadComplete, LoadLevel};
use crate::state::game_state::{write_save, ActiveSave, GameSaveState, LevelClearState};

pub struct PlayerDied;

/// Active Player State
pub struct Player {
  pub height_adjust: f32,
  pub grounded: bool,
  pub jump_boost_time: f32,
  pub jump_in_progress: bool,
  pub outside_ground_bounds: bool,
  pub on_moving_entity: Option<Entity>,
  pub respawn_pos: Vec2,
}

impl Player {
  pub const JUMP_BOOST_TIME: f32 = 0.35;
  pub const NORMAL_FALL_SPEED: f32 = 2.25;
  pub const SLOW_FALL_SPEED: f32 = 1.25;

  pub fn new(respawn_pos: Vec2) -> Self {
    Self {
      height_adjust: 0.25,
      grounded: true,
      jump_boost_time: Self::JUMP_BOOST_TIME,
      jump_in_progress: false,
      outside_ground_bounds: false,
      on_moving_entity: None,
      respawn_pos,
    }
  }
}

impl Attribute for Player {
  const KEY: &'static str = "player";

  fn build(commands: &mut Commands, target: Entity, position: Vec2, _: Vec<i32>) {
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
      .insert(Player::new(position.into()))
      .insert_bundle(rigid_body)
      .insert_bundle(collider)
      .insert(RigidBodyPositionSync::Interpolated { prev_pos: None });
  }
}

/// Consumes [PlayerDied] tags and respawns the player.
pub fn on_death_system(
  mut commands: Commands,
  death_tags: Query<(Entity, &PlayerDied)>,
  mut player: Query<(&mut RigidBodyPosition, &Player)>,
) {
  if let Ok((mut pos, player)) = player.single_mut() {
    death_tags.for_each(|(ent, _)| {
      commands.entity(ent).despawn();
      pos.position.translation = player.respawn_pos.into();
    });
  }
}

/// Consumes [PlayerReachedCheckpoint] tags and sets the new player respawn
/// point.
pub fn on_checkpoint_system(
  mut commands: Commands,
  checkpoint_reached: Query<(Entity, &Checkpoint), With<PlayerReachedCheckpoint>>,
  mut player: Query<&mut Player>,
  audio: Res<Audio>,
  sfx_handles: Res<SfxHandles>,
  channels: Res<AudioChannels>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  mut active_save: ResMut<ActiveSave>,
) {
  if let Ok(mut player) = player.single_mut() {
    checkpoint_reached.for_each(|(ent, checkpoint)| {
      if player.respawn_pos != checkpoint.0 {
        player.respawn_pos = checkpoint.0;
        audio.play_in_channel(sfx_handles.checkpoint.clone(), &channels.sfx.0);
        if let Some(save) = &mut active_save.0 {
          if let Ok(level) = loaded_level.single() {
            save.level_clears.insert(
              GameSaveState::key(level.0),
              LevelClearState::AtCheckpoint(checkpoint.0.x, checkpoint.0.y),
            );
            write_save(save);
          }
        }
      }

      commands.entity(ent).remove::<PlayerReachedCheckpoint>();
    });
  }
}
