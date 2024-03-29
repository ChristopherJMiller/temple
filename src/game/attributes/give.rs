//! Gives a player the component provided within the parameter.
//! `give(component_name(params...))`

use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::{Attribute, Dash, Player};
use crate::game::collision::{ContactQuery, ContactSubscription};
use crate::game::collision_groups::*;
use crate::level::LevelId;

#[derive(Component)]
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
      shape: ColliderShape::cuboid(0.5, 0.5).into(),
      material: ColliderMaterialComponent::default(),
      flags: ColliderFlags {
        collision_groups: DETECTS_PLAYER_GROUP,
        solver_groups: NONE_GROUP,
        active_events: ActiveEvents::CONTACT_EVENTS,
        ..Default::default()
      }
      .into(),
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

pub fn on_give_system(mut commands: Commands, player: Query<Entity, With<Player>>, goal: ContactQuery<Give>) {
  if let Ok(player) = player.get_single() {
    goal.for_each(|(ent, give)| {
      commands.entity(player).insert(give.build_component());
      commands.entity(ent).despawn();
    });
  }
}
