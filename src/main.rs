use crate::sprite::SpritePlugin;
use bevy::prelude::*;

use crate::input::InputPlugin;

pub mod input;
pub mod game;
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
    .run();
}
