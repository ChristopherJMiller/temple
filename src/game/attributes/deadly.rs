//! Makes sprite deadly to the player. `deadly()`

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::{Attribute, Player};
use crate::game::collision::{ContactSubscription, ContactTagQuery, PlayerContacted};
use crate::game::collision_groups::*;
use crate::level::load::{LevelLoadComplete, LoadLevel, TransitionLevel};
use crate::level::LevelId;

#[derive(Component)]
pub struct Deadly;

impl Attribute for Deadly {
  const KEY: &'static str = "deadly";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, _: Vec<ParseArgumentItem>) {
    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(0.5, 0.5).into(),
      material: ColliderMaterialComponent::default(),
      flags: ColliderFlags {
        collision_groups: DEADLY_GROUP,
        solver_groups: NONE_GROUP,
        active_events: ActiveEvents::CONTACT_EVENTS,
        ..Default::default()
      }
      .into(),
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

/// Consumes [PlayerContacted] tags and respawns the player.
pub fn on_death_system(
  mut commands: Commands,
  deadly_contacted: ContactTagQuery<Deadly>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  mut player: Query<(&mut RigidBodyPositionComponent, &Player)>,
) {
  if let Ok((mut pos, player)) = player.get_single_mut() {
    deadly_contacted.for_each(|ent| {
      let level_id = loaded_level.get_single().unwrap().0;
      if player.respawn_level != level_id {
        commands.spawn().insert(TransitionLevel(player.respawn_level));
      } else {
        pos.position.translation = player.respawn_pos.into();
      }

      commands.entity(ent).remove::<PlayerContacted>();
    });
  }
}
