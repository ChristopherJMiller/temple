use bevy::prelude::*;

// TODO: Allow for parameter input from attribute e.g. moving_platform(x, 5.0)
/// Sprite Type Attribute trait
trait Attribute {
  const KEY: &'static str;
  fn build(commands: &mut Commands, target: Entity, position: Vec2, params: Vec<i32>);
}

struct AttributeEntry(String, Vec<i32>);

fn derive_attribute_entry(entry: String) -> AttributeEntry {
  // e.g. attribute(...)
  if entry.contains("(") && entry.contains(")") {
    let mut entry_copy = entry.clone();
    entry_copy.retain(|x| !x.is_whitespace());
    entry_copy.pop();

    // Splits attribute symbol and parameters, removes empty string artifacts if had no parameters e.g. attribute()
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
    let parameters: Vec<i32> = entry_list[1].split(",").map(|x| x.to_string().parse::<i32>().unwrap_or_else(|_| panic!("Attempted to unwrap attribute parameter but was not a signed integer, found {}", x))).collect();

    AttributeEntry(entry_list[0].to_string(), parameters)
  } else {
    // e.g. attribute
    AttributeEntry(entry, Vec::new())
  }
}

pub fn build_attribute(attribute: String, commands: &mut Commands, target: Entity, position: Vec2) {
  let entry = derive_attribute_entry(attribute);
  match entry.0.as_str() {
    Player::KEY => Player::build(commands, target, position, entry.1),
    Solid::KEY => Solid::build(commands, target, position, entry.1),
    MovingSprite::KEY => MovingSprite::build(commands, target, position, entry.1),
    _ => panic!("Attempted to load invalid attribute with name {}", entry.0),
  }
}

mod player;
mod solid;
mod moving;

pub use player::*;
pub use solid::*;
pub use moving::*;

pub struct AttributePlugin;

impl Plugin for AttributePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(moving_system.system().label(MovingAttributeSystemSteps::ApplyDeltaTranslation))
      .add_system(move_player.system().after(MovingAttributeSystemSteps::ApplyDeltaTranslation));
  }
}
