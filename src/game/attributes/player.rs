//! Represents the player. `player()`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::game::collision_groups::*;
use crate::level::LevelId;

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
