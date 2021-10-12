//! Sets the players checkpoint. `checkpoint(optional x offset, optional y offset)`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Attribute;
use crate::game::collision_groups::*;
use crate::sprite::SPRITE_SIZE;

pub struct PlayerReachedCheckpoint;
pub struct Checkpoint(pub Vec2);

impl Attribute for Checkpoint {
  const KEY: &'static str = "checkpoint";

  fn build(commands: &mut Commands, target: Entity, position: Vec2, params: Vec<i32>) {
    let player_offset = if params.len() > 0 {
      let x_offset = params.get(0);
      let y_offset = params.get(1);

      if x_offset.is_none() || y_offset.is_none() {
        panic!("Attempted to construct a checkpoint with an offset, but provided too few arguments!");
      } else {
        Vec2::new(*x_offset.unwrap() as f32, *y_offset.unwrap() as f32)
      }

    } else {
      Vec2::ZERO
    };

    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(SPRITE_SIZE as f32 / 2.0, SPRITE_SIZE as f32 / 2.0),
      material: ColliderMaterial::default(),
      flags: ColliderFlags {
        collision_groups: DETECTS_PLAYER_GROUP,
        solver_groups: NONE_GROUP,
        active_events: ActiveEvents::CONTACT_EVENTS,
        ..Default::default()
      },
      ..Default::default()
    };

    commands
      .entity(target)
      .insert(Checkpoint(position + (player_offset * SPRITE_SIZE as f32)))
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}
