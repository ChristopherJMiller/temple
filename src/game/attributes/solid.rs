use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::collision_groups::*;
use crate::level::SPRITE_SIZE;

use super::Attribute;

pub struct Solid;

impl Attribute for Solid {
  const KEY: &'static str = "solid";

  fn build(commands: &mut Commands, target: Entity, position: Vec2) {
    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(SPRITE_SIZE as f32 / 2.0, SPRITE_SIZE as f32 / 2.0),
      material: ColliderMaterial::default(),
      flags: ColliderFlags {
        collision_groups: SOLID_GROUP,
        solver_groups: SOLID_GROUP,
        ..Default::default()
      },
      ..Default::default()
    };

    commands
      .entity(target)
      .insert(Solid)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}
