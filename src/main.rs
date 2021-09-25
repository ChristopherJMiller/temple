use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::GamePlugins;
use input::InputPlugin;
use level::load::LoadLevel;
use level::LevelPlugin;
use sprite::SpritePlugin;
use util::cli::{get_cli_args, CliArgs};
use util::files::verify_files;
use util::settings::{get_game_file, Version};

mod game;
mod input;
mod level;
mod sprite;
mod util;

/// Game Entrypoint
fn main() {
  // Version supplied by build.rs
  let version = String::from(env!("VERSION"));
  verify_files();

  // Once files are verified, get the main GameFile and use it to parse incoming
  // cli args
  let game_file = get_game_file();
  let cli_args = get_cli_args(version.clone(), &game_file);

  // Bevy Bootstrapping
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
    // 3rd Party Plugins
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierRenderPlugin)

    // Game Plugins
    .add_plugin(InputPlugin)
    .add_plugin(SpritePlugin)
    .add_plugin(LevelPlugin)
    .add_plugins(GamePlugins)
    .add_startup_system(handle_cli_args.system())
    .run();
}

/// Consumes incoming CLI Arguments
fn handle_cli_args(mut commands: Commands, cli_args: Res<CliArgs>) {
  // --load <level>
  if let Some(level) = cli_args.load_level {
    commands.spawn().insert(LoadLevel(level));
  }
}
