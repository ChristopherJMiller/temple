//! Handles contant and interaction events from rapier

use bevy::prelude::*;
use bevy_rapier2d::physics::IntoEntity;
use bevy_rapier2d::prelude::ContactEvent::{self, Started};

use super::attributes::Player;

/// Subscribes an Entity to collision events. Can be paired with attributes to
/// be consumed with other systems.
#[derive(Component)]
pub struct ContactSubscription;

/// Tags entity that a player contacted it.
#[derive(Component)]
pub struct PlayerContacted;

/// Player contact query, for attributes that are stateless.
pub type ContactTagQuery<'s, 'world, 'state, K> = Query<'world, 'state, Entity, (With<K>, With<PlayerContacted>)>;

// Player contact query, for attributes that need access to state within the
// system.
pub type ContactQuery<'s, 'world, 'state, K> = Query<'world, 'state, (Entity, &'s K), With<PlayerContacted>>;

/// Check if an entity is subscribed to collision events, and hasn't received
/// one yet.
fn should_receive_event(
  entity: Entity,
  subscribed: &Query<&ContactSubscription>,
  already_collided: &Query<&PlayerContacted>,
) -> bool {
  return subscribed.get_component::<ContactSubscription>(entity).is_ok()
    && already_collided.get_component::<PlayerContacted>(entity).is_err();
}

/// Handles incoming contact events.
pub fn handle_collision_events(
  mut commands: Commands,
  mut contact_events: EventReader<ContactEvent>,
  player_query: Query<&Player>,
  subscribed: Query<&ContactSubscription>,
  already_collided: Query<&PlayerContacted>,
) {
  for contact_event in contact_events.iter() {
    if let Started(a, b) = contact_event {
      if player_query.get_component::<Player>(a.entity()).is_ok() {
        if should_receive_event(b.entity(), &subscribed, &already_collided) {
          commands.entity(b.entity()).insert(PlayerContacted);
        }
      } else if player_query.get_component::<Player>(b.entity()).is_ok() {
        if should_receive_event(a.entity(), &subscribed, &already_collided) {
          commands.entity(a.entity()).insert(PlayerContacted);
        }
      }
    }
  }
}

/// [Plugin] for handling collision events from rapier.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(handle_collision_events);
  }
}
