//! Temple is a platform built with the [Bevy Engine](https://docs.rs/bevy/) and has a focus on
//! configurability.
//!
//! Check out the [github](https://github.com/ChristopherJMiller/temple) for more info.

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;
use game::GamePlugins;
use input::InputPlugin;
use level::LevelPlugin;
use sprite::SpritePlugin;
use ui::UiPlugin;
use util::cli::{get_cli_args, handle_cli_args};
use util::files::verify_files;
use util::settings::get_game_file;

mod game;
mod input;
mod level;
mod sprite;
mod ui;
mod util;

/// Game version. For dev builds, this is a timestamp.
pub const VERSION: &str = env!("VERSION");

/// Game Entrypoint
fn main() {
  // Version supplied by build.rs
  let version = VERSION.to_string();
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
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .insert_resource(game_file)
    .insert_resource(cli_args)
    // 3rd Party Plugins
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(EguiPlugin)
    .add_plugin(AudioPlugin)

    // Game Plugins
    .add_plugin(InputPlugin)
    .add_plugin(SpritePlugin)
    .add_plugin(LevelPlugin)
    .add_plugins(GamePlugins)
    .add_plugin(UiPlugin)
    .add_startup_system(handle_cli_args.system())
    .run();
}
