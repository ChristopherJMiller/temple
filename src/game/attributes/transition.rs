//! Transitions the player to a new level. `trans(new level id)`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::game::collision::ContactSubscription;
use crate::game::collision_groups::*;
use crate::level::LevelId;

pub struct Transition(pub LevelId);

fn parse_params<'a>(params: Vec<ParseArgumentItem>) -> Result<LevelId, &'a str> {
  if let Some(item) = params.get(0) {
    if let ParseArgumentItem::Number(id) = item {
      if let Ok(id) = u32::try_from(*id) {
        Ok(id)
      } else {
        Err("Argument provided must be a positive number!")
      }
    } else {
      Err("Argument provided for Tranistion id was not a number!")
    }
  } else {
    Err("No Argument was Provided to Transition Attribute!")
  }
}

impl Attribute for Transition {
  const KEY: &'static str = "trans";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, params: Vec<ParseArgumentItem>) {
    let id = parse_params(params);

    if let Err(err) = id {
      panic!("{}", err);
    }

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
      .insert(ContactSubscription)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete)
      .insert(Transition(id.unwrap()));
  }
}
