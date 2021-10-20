//! Represents the player. `player()`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Attribute, Checkpoint, PlayerReachedCheckpoint};
use crate::game::collision_groups::*;

pub struct PlayerDied;

/// Active Player State
pub struct Player {
  pub height_adjust: f32,
  pub grounded: bool,
  pub jump_in_progress: bool,
  pub outside_ground_bounds: bool,
  pub on_moving_entity: Option<Entity>,
  pub respawn_pos: Vec2,
}

impl Player {
  pub fn new(respawn_pos: Vec2) -> Self {
    Self {
      height_adjust: 0.25,
      grounded: true,
      jump_in_progress: false,
      outside_ground_bounds: false,
      on_moving_entity: None,
      respawn_pos
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
        gravity_scale: 0.5,
        ..Default::default()
      },
      damping: RigidBodyDamping {
        linear_damping: 1.0,
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
      .insert(ColliderPositionSync::Discrete);
  }
}

/// Consumes [PlayerDied] tags and respawns the player.
pub fn on_death_system(mut commands: Commands, death_tags: Query<(Entity, &PlayerDied)>, mut player: Query<(&mut RigidBodyPosition, &Player)>) {
  if let Ok((mut pos, player)) = player.single_mut() {
    death_tags.for_each(|(ent, _)| {
      commands.entity(ent).despawn();
      pos.position.translation = player.respawn_pos.into();
    });
  }
}

/// Consumes [PlayerReachedCheckpoint] tags and sets the new player respawn point.
pub fn on_checkpoint_system(mut commands: Commands, checkpoint_reached: Query<(Entity, &Checkpoint), With<PlayerReachedCheckpoint>>, mut player: Query<&mut Player>) {
  if let Ok(mut player) = player.single_mut() {
    checkpoint_reached.for_each(|(ent, checkpoint)| {
      player.respawn_pos = checkpoint.0;
      commands.entity(ent).remove::<PlayerReachedCheckpoint>();
    });
  }
}
