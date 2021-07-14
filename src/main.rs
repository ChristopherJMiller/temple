use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::GamePlugins;
use crate::input::InputPlugin;
use crate::level::{LevelPlugin, LoadLevel};
use crate::sprite::SpritePlugin;
use crate::util::cli::{get_cli_args, CliArgs};
use crate::util::files::verify_files;
use crate::util::settings::{get_game_file, Version};

pub mod game;
pub mod input;
pub mod level;
pub mod sprite;
pub mod util;

fn main() {
  let version = String::from(env!("VERSION"));
  verify_files();
  let game_file = get_game_file();
  let cli_args = get_cli_args(version.clone(), &game_file);

  App::build()
    .insert_resource(WindowDescriptor {
      title: game_file.title.clone(),
      width: 1170.,
      height: 1024.,
      vsync: true,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 0.0)))
    .insert_resource(Version(version))
    .insert_resource(game_file)
    .insert_resource(cli_args)
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierRenderPlugin)
    .add_plugin(InputPlugin)
    .add_plugin(SpritePlugin)
    .add_plugin(LevelPlugin)
    .add_plugins(GamePlugins)
    .add_startup_system(handle_cli_args.system())
    .run();
}

fn handle_cli_args(mut commands: Commands, cli_args: Res<CliArgs>) {
  if let Some(level) = cli_args.load_level {
    commands.spawn().insert(LoadLevel(level));
  }
}
