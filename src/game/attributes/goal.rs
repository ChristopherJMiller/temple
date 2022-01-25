//! Completes an exit for the active level. `goal(exit number)`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::game::collision::ContactSubscription;
use crate::game::collision_groups::*;
use crate::level::LevelId;

pub struct Goal(pub usize);

fn parse_params(params: Vec<ParseArgumentItem>) -> usize {
  let arg = params
    .get(0)
    .unwrap_or_else(|| panic!("Goal attribute created with no exit number parameter!"));
  if let ParseArgumentItem::Number(exit) = arg {
    exit
      .clone()
      .try_into()
      .unwrap_or_else(|_| panic!("Invalid number provided for exit number!"))
  } else {
    panic!("Non-number parameter provided to goal attribute!");
  }
}

impl Attribute for Goal {
  const KEY: &'static str = "goal";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, params: Vec<ParseArgumentItem>) {
    let exit_number = parse_params(params);

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
      .insert(Goal(exit_number))
      .insert(ContactSubscription)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}
