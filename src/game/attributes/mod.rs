//! Attribute definitions and systems.
//!
//! # About Attributes
//! Attributes are Temple's translation of a Bevy component into a
//! manifest-defined space for use with sprites (see [crate::sprite]).
//! In most scenario's, they are simply tags to apply a system, but attributes
//! additionally have the ability to take parameters to generalize their
//! functionality.

use bevy::prelude::*;

macro_rules! attribute_build_tree {
  ($incoming_attr:expr, $commands:expr, $target:expr, $level:expr, $position:expr, [$($attribute:path),+]) => {
    let entry = AttributeEntry::from($incoming_attr);
    match entry.0.as_str() {
      $(<$attribute>::KEY => <$attribute>::build($commands, $target, $level, $position, entry.1),)+
      _ => panic!("Attempted to load invalid attribute with name {}", entry.0),
    }
  };
}

/// Attribute, as used with a [crate::sprite::SpriteType]
pub trait Attribute {
  const KEY: &'static str;
  fn build(commands: &mut Commands, target: Entity, level: LevelId, position: Vec2, params: Vec<ParseArgumentItem>);
}

/// Constructs attribute onto a given [Entity]. Used during level load (see
/// [crate::level::load::load_level]).
pub fn build_attribute(attribute: String, commands: &mut Commands, target: Entity, level: LevelId, position: Vec2) {
  attribute_build_tree!(
    attribute,
    commands,
    target,
    level,
    position,
    [
      Player,
      Solid,
      MovingSprite,
      Deadly,
      Checkpoint,
      Transition,
      Goal,
      Dash,
      GivableAttribute
    ]
  );
}

mod checkpoint;
mod dash;
mod deadly;
mod give;
mod goal;
mod lex;
mod moving;
mod player;
mod solid;
mod transition;

pub use checkpoint::*;
pub use dash::*;
pub use deadly::*;
pub use give::*;
pub use goal::*;
pub use moving::*;
pub use player::*;
pub use solid::*;
pub use transition::*;

use self::lex::{AttributeEntry, ParseArgumentItem};
use super::physics::PlayerSimulationSteps;
use crate::level::LevelId;

/// [Plugin] for attributes
pub struct AttributePlugin;

impl Plugin for AttributePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(moving_system.label(MovingAttributeSystemSteps::ApplyDeltaTranslation))
      .add_system(
        move_player
          .after(MovingAttributeSystemSteps::ApplyDeltaTranslation)
          .after(PlayerSimulationSteps::ApplyMoving),
      )
      .add_system(on_death_system)
      .add_system(on_checkpoint_system)
      .add_system(on_transition_system)
      .add_system(on_goal_system)
      .add_system(on_give_system);
  }
}
