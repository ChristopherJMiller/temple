use bevy::prelude::*;

use crate::input::InputPlugin;
use crate::level::LevelPlugin;
use crate::sprite::SpritePlugin;

pub mod game;
pub mod input;
pub mod level;
pub mod sprite;

fn main() {
  App::build()
    .insert_resource(WindowDescriptor {
      title: "Temple".to_string(),
      width: 1600.,
      height: 900.,
      vsync: true,
      ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(InputPlugin)
    .add_plugin(SpritePlugin)
    .add_plugin(LevelPlugin)
    .run();
}
