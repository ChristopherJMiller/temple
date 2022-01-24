//! Makes sprite deadly to the player. `deadly()`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::game::collision::ContactSubscription;
use crate::game::collision_groups::*;
use crate::level::LevelId;

pub struct Deadly;

impl Attribute for Deadly {
  const KEY: &'static str = "deadly";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, _: Vec<ParseArgumentItem>) {
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
      .insert(Deadly)
      .insert(ContactSubscription)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}
