use bevy::prelude::*;

use crate::input::InputPlugin;
use crate::level::{LoadLevel, LevelPlugin};
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
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins)
    .add_plugin(InputPlugin)
    .add_plugin(SpritePlugin)
    .add_plugin(LevelPlugin)
    .add_startup_system(dev_load_level.system())
    .run();
}

fn dev_load_level(mut commands: Commands) {
  let mut camera = OrthographicCameraBundle::new_2d();
  camera.orthographic_projection.scale = 1.0 / 4.0;

  commands.spawn_bundle(camera);
  commands.spawn().insert(LoadLevel(0));
}
