//! Makes sprite solid to the player. `solid()`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::game::collision_groups::*;
use crate::level::LevelId;

#[derive(Component)]
pub struct Solid;

impl Attribute for Solid {
  const KEY: &'static str = "solid";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, _: Vec<ParseArgumentItem>) {
    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(0.5, 0.5).into(),
      material: ColliderMaterialComponent::default(),
      flags: ColliderFlags {
        collision_groups: SOLID_GROUP,
        solver_groups: SOLID_GROUP,
        ..Default::default()
      }
      .into(),
      ..Default::default()
    };

    commands.entity(target).insert(Solid).insert_bundle(collider);
  }
}
