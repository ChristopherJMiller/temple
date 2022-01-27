//! Gives a player the component provided within the parameter. `give(component_name(params...))`

use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::{Attribute, Dash};
use crate::game::collision::ContactSubscription;
use crate::game::collision_groups::*;
use crate::level::LevelId;

pub struct Give(Vec<ParseArgumentItem>);

/// Typeless Give attribute, used to build [Give]
pub struct GivableAttribute;

impl Give {
  pub fn build_component(&self) -> impl Component {
    if let Some(ParseArgumentItem::Str(key)) = self.0.get(0) {
      match key.as_str() {
        Dash::KEY => Dash::default(),
        _ => panic!("Attempted to use an invalid attribute when giving!"),
      }
    } else {
      panic!("Attempted to use give attribute without component provided")
    }
  }
}

impl Attribute for GivableAttribute {
  const KEY: &'static str = "give";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, params: Vec<ParseArgumentItem>) {
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
      .insert(Give(params))
      .insert(ContactSubscription)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}
