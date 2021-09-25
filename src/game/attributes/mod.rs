use bevy::prelude::*;

// TODO: Allow for parameter input from attribute e.g. moving_platform(x, 5.0)
/// Sprite Type Attribute trait
trait Attribute {
  const KEY: &'static str;
  fn build(commands: &mut Commands, target: Entity, position: Vec2);
}

pub fn build_attribute(name: String, commands: &mut Commands, target: Entity, position: Vec2) {
  match name.as_str() {
    Player::KEY => Player::build(commands, target, position),
    Solid::KEY => Solid::build(commands, target, position),
    _ => panic!("Attempted to load invalid attribute with name {}", name),
  }
}

mod player;
mod solid;

pub use player::*;
pub use solid::*;
