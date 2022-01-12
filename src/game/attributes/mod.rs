//! Attribute definitions and systems.
//!
//! # About Attributes
//! Attributes are Temple's translation of a Bevy component into a
//! manifest-defined space for use with sprites (see [crate::sprite]).
//! In most scenario's, they are simply tags to apply a system, but attributes
//! additionally have the ability to take parameters to generalize their
//! functionality.

use bevy::prelude::*;

/// Attribute, as used with a [crate::sprite::SpriteType]
pub trait Attribute {
  const KEY: &'static str;
  fn build(commands: &mut Commands, target: Entity, position: Vec2, params: Vec<i32>);
}

// TODO: Allow parameters to be strings. Attributes should individually handle
// parsing.
/// Attribute with supplied parameters.
struct AttributeEntry(String, Vec<i32>);

/// Parses attribute string as a key and supplied parameters.
fn derive_attribute_entry(entry: String) -> AttributeEntry {
  // e.g. attribute(...)
  if entry.contains("(") && entry.contains(")") {
    let mut entry_copy = entry.clone();
    entry_copy.retain(|x| !x.is_whitespace());
    entry_copy.pop();

    // Splits attribute symbol and parameters, removes empty string artifacts if had
    // no parameters e.g. attribute()
    let entry_list: Vec<&str> = entry_copy.split("(").filter(|&x| x.len() > 0).collect();

    // e.g. attribute(())
    if entry_list.len() > 2 {
      panic!("Attempted to load level with invalid attribute entry {}", entry);
    }

    // e.g. attribute()
    if entry_list.len() == 1 {
      return AttributeEntry(entry_list[0].to_string(), Vec::new());
    }

    // e.g. attribute(1,...)
    let parameters: Vec<i32> = entry_list[1]
      .split(",")
      .map(|x| {
        x.to_string().parse::<i32>().unwrap_or_else(|_| {
          panic!(
            "Attempted to unwrap attribute parameter but was not a signed integer, found {}",
            x
          )
        })
      })
      .collect();

    AttributeEntry(entry_list[0].to_string(), parameters)
  } else {
    // e.g. attribute
    AttributeEntry(entry, Vec::new())
  }
}

pub const REGISTERED_ATTRIBUTES: [&str; 5] = [Player::KEY, Solid::KEY, MovingSprite::KEY, Deadly::KEY, Checkpoint::KEY];

/// Constructs attribute onto a given [Entity]. Used during level load (see
/// [crate::level::load::load_level]).
pub fn build_attribute(attribute: String, commands: &mut Commands, target: Entity, position: Vec2) {
  let entry = derive_attribute_entry(attribute);
  match entry.0.as_str() {
    Player::KEY => Player::build(commands, target, position, entry.1),
    Solid::KEY => Solid::build(commands, target, position, entry.1),
    MovingSprite::KEY => MovingSprite::build(commands, target, position, entry.1),
    Deadly::KEY => Deadly::build(commands, target, position, entry.1),
    Checkpoint::KEY => Checkpoint::build(commands, target, position, entry.1),
    _ => panic!("Attempted to load invalid attribute with name {}", entry.0),
  }
}

mod checkpoint;
mod deadly;
mod moving;
mod player;
mod solid;

pub use checkpoint::*;
pub use deadly::*;
pub use moving::*;
pub use player::*;
pub use solid::*;

use super::physics::PlayerSimulationSteps;

/// [Plugin] for attributes
pub struct AttributePlugin;

impl Plugin for AttributePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(
        moving_system
          .system()
          .label(MovingAttributeSystemSteps::ApplyDeltaTranslation),
      )
      .add_system(
        move_player
          .system()
          .after(MovingAttributeSystemSteps::ApplyDeltaTranslation)
          .after(PlayerSimulationSteps::ApplyMoving),
      )
      .add_system(on_death_system.system())
      .add_system(on_checkpoint_system.system());
  }
}
