//! Makes sprite solid to the player. `solid()`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Attribute;
use crate::game::collision_groups::*;

pub struct Solid;

impl Attribute for Solid {
  const KEY: &'static str = "solid";

  fn build(commands: &mut Commands, target: Entity, position: Vec2, _: Vec<i32>) {
    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(0.5, 0.5),
      material: ColliderMaterial::default(),
      flags: ColliderFlags {
        collision_groups: SOLID_GROUP,
        solver_groups: SOLID_GROUP,
        ..Default::default()
      },
      ..Default::default()
    };

    commands.entity(target).insert(Solid).insert_bundle(collider);
  }
}
