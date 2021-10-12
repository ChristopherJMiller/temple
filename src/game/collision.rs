//! Handles contant and interaction events from rapier

use bevy::prelude::*;
use bevy_rapier2d::physics::IntoEntity;
use bevy_rapier2d::prelude::ContactEvent::{self, Started};

use super::attributes::{Attribute, PlayerReachedCheckpoint, Checkpoint, Deadly, Player, PlayerDied};

/// Given an [Entity], determines what attribute it contains if any.
fn determine_possible_tag_collision(entity: Entity, has_deadly: &Query<&Deadly>, has_checkpoint: &Query<&Checkpoint>) -> Option<String> {
  if has_deadly.get_component::<Deadly>(entity).is_ok() {
    return Some(Deadly::KEY.to_string());
  } else if has_checkpoint.get_component::<Checkpoint>(entity).is_ok() {
    return Some(Checkpoint::KEY.to_string());
  }

  return None;
}

/// Consumes the determined attribute that should be accounted for as a collision event.
fn on_contact_with_player(commands: &mut Commands, tag: String, collision_entity: Entity) {
  match tag.as_str() {
    Deadly::KEY => { 
      commands.spawn().insert(PlayerDied);
    },
    Checkpoint::KEY => {
      commands.entity(collision_entity).insert(PlayerReachedCheckpoint);
    }
    _ => {},
  };
}

/// Handles incoming contact events.
pub fn handle_collision_events(
  mut commands: Commands,
  mut contact_events: EventReader<ContactEvent>,
  player_query: Query<&Player>,
  has_deadly: Query<&Deadly>,
  has_checkpoint: Query<&Checkpoint>
) {
  for contact_event in contact_events.iter() {
    if let Started(a, b) = contact_event {
      if player_query.get_component::<Player>(a.entity()).is_ok() {
        if let Some(tag) = determine_possible_tag_collision(b.entity(), &has_deadly, &has_checkpoint) {
          on_contact_with_player(&mut commands, tag, b.entity());
        }
      } else if player_query.get_component::<Player>(b.entity()).is_ok() {
        if let Some(tag) = determine_possible_tag_collision(a.entity(), &has_deadly, &has_checkpoint) {
          on_contact_with_player(&mut commands, tag, a.entity());
        }
      }
    }
  }
}

/// [Plugin] for handling collision events from rapier.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(handle_collision_events.system());
  }
}
