
use bevy::prelude::*;
use bevy_rapier2d::physics::IntoEntity;
use bevy_rapier2d::prelude::ContactEvent::{self, Started};

use super::attributes::{Attribute, Deadly, Player, PlayerDied};

fn determine_possible_tag_collision(entity: Entity, deadly_query: &Query<&Deadly>) -> Option<String> {
  if deadly_query.get_component::<Deadly>(entity).is_ok() {
    return Some(Deadly::KEY.to_string());
  }

  return None;
}

fn on_contact_with_player(commands: &mut Commands, tag: String) {
  match tag.as_str() {
    Deadly::KEY => { 
      commands.spawn().insert(PlayerDied);
    },
    _ => {},
  };
}

pub fn handle_collision_events(
  mut commands: Commands,
  mut contact_events: EventReader<ContactEvent>,
  player_query: Query<&Player>,
  deadly_query: Query<&Deadly>,
) {
  for contact_event in contact_events.iter() {
    if let Started(a, b) = contact_event {
      if player_query.get_component::<Player>(a.entity()).is_ok() {
        if let Some(tag) = determine_possible_tag_collision(b.entity(), &deadly_query) {
          on_contact_with_player(&mut commands, tag);
        }
      } else if player_query.get_component::<Player>(b.entity()).is_ok() {
        if let Some(tag) = determine_possible_tag_collision(a.entity(), &deadly_query) {
          on_contact_with_player(&mut commands, tag);
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
