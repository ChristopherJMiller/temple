use bevy::prelude::*;

use crate::input::InputPlugin;

pub mod input;

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
    .run();
}
