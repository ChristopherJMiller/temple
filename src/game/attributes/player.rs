use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Attribute;
use crate::game::collision_groups::*;
use crate::sprite::SPRITE_SIZE;

/// Active Player State
pub struct Player {
  pub height_adjust: f32,
  pub grounded: bool,
  pub jump_in_progress: bool,
  pub outside_ground_bounds: bool,
}

impl Default for Player {
  fn default() -> Player {
    Player {
      height_adjust: 0.25,
      grounded: false,
      jump_in_progress: false,
      outside_ground_bounds: false,
    }
  }
}

impl Attribute for Player {
  const KEY: &'static str = "player";

  fn build(commands: &mut Commands, target: Entity, position: Vec2) {
    let rigid_body = RigidBodyBundle {
      position: position.into(),
      mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
      forces: RigidBodyForces {
        gravity_scale: 5.0,
        ..Default::default()
      },
      damping: RigidBodyDamping {
        linear_damping: 0.5,
        ..Default::default()
      },
      ..Default::default()
    };

    let collider = ColliderBundle {
      position: position.into(),
      material: ColliderMaterial {
        friction: 0.0,
        ..Default::default()
      },
      shape: ColliderShape::ball(SPRITE_SIZE as f32 / 2.0),
      flags: ColliderFlags {
        collision_groups: PLAYER_GROUP,
        solver_groups: PLAYER_GROUP,
        ..Default::default()
      },
      ..Default::default()
    };

    commands
      .entity(target)
      .insert(Player::default())
      .insert_bundle(rigid_body)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}
