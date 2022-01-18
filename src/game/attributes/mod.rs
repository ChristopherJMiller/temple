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
  fn build(commands: &mut Commands, target: Entity, position: Vec2, params: Vec<ParseArgumentItem>);
}

/// Constructs attribute onto a given [Entity]. Used during level load (see
/// [crate::level::load::load_level]).
pub fn build_attribute(attribute: String, commands: &mut Commands, target: Entity, position: Vec2) {
  let entry = AttributeEntry::from(attribute);
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
mod lex;
mod moving;
mod player;
mod solid;

pub use checkpoint::*;
pub use deadly::*;
pub use moving::*;
pub use player::*;
pub use solid::*;

use self::lex::{AttributeEntry, ParseArgumentItem};
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
