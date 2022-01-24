//! Sets the players checkpoint. `checkpoint(optional x offset, optional y
//! offset)`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::game::collision::ContactSubscription;
use crate::game::collision_groups::*;
use crate::level::LevelId;
use crate::level::config::SPRITE_SIZE;

pub struct Checkpoint(pub Vec2);

impl Attribute for Checkpoint {
  const KEY: &'static str = "checkpoint";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, params: Vec<ParseArgumentItem>) {
    let player_offset = if params.len() > 0 {
      let x_offset = params.get(0);
      let y_offset = params.get(1);

      if x_offset.is_none() || y_offset.is_none() {
        panic!("Attempted to construct a checkpoint with an offset, but provided too few arguments!");
      } else {
        if let Some(ParseArgumentItem::Number(x)) = x_offset {
          if let Some(ParseArgumentItem::Number(y)) = y_offset {
            Vec2::new(*x as f32, *y as f32)
          } else {
            panic!("y coord not found!");
          }
        } else {
          panic!("x coord not found!");
        }
      }
    } else {
      Vec2::ZERO
    };

    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(0.5, 0.5),
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
      .insert(ContactSubscription)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}
