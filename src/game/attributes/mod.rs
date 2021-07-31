use bevy::prelude::*;

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

mod solid;
mod player;

pub use solid::*;
pub use player::*;
