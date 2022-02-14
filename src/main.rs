//! Temple is a platform built with the [Bevy Engine](https://docs.rs/bevy/) and has a focus on
//! configurability.
//!
//! Check out the [github](https://github.com/ChristopherJMiller/temple) for more info.

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;
use editor::EditorPlugins;
use game::sfx::SfxPlugin;
use game::GamePlugins;
use input::InputPlugin;
use level::save::LevelSavePlugin;
use level::LevelPlugin;
use state::game_state::TempleState;
use state::settings::Settings;
use state::StatePlugin;
use ui::UiPlugin;
use util::cli::{get_cli_args, handle_cli_args, CliArgs};
use util::files::verify_files;
use util::settings::{get_game_file, GameFile};

mod editor;
mod game;
mod input;
mod level;
mod state;
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
  let settings = Settings::from_file();

  if cli_args.edit_mode {
    start_editor(game_file, cli_args, settings);
  } else {
    start_game(game_file, cli_args, settings);
  }
}

fn build_base_app(app: &mut App, game_file: GameFile, cli_args: CliArgs, settings: Settings) {
  let (width, height) = settings.scale.into();

  app
    .insert_resource(WindowDescriptor {
      title: game_file.title.clone(),
      width: width as f32,
      height: height as f32,
      scale_factor_override: Some(settings.scale.get_ui_scale()),
      vsync: true,
      resizable: false,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(game_file)
    .insert_resource(cli_args)
    .insert_resource(settings)

    // 3rd Party Plugins
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(EguiPlugin)
    .add_plugin(AudioPlugin)

    // Game Plugins
    .add_plugin(StatePlugin)
    .add_plugin(InputPlugin)
    .add_plugin(LevelPlugin)
    .add_plugin(UiPlugin)
    .add_startup_system(handle_cli_args);
}

/// Start Game
fn start_game(game_file: GameFile, cli_args: CliArgs, settings: Settings) {
  let mut app = App::new();
  build_base_app(&mut app, game_file, cli_args, settings);

  app
    // 3rd Party Plugins
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())

    // Game Plugins
    .add_plugins(GamePlugins)

    // Init Temple State
    .insert_resource(TempleState::default())
    .run();
}

fn start_editor(game_file: GameFile, cli_args: CliArgs, settings: Settings) {
  let mut app = App::new();
  build_base_app(&mut app, game_file, cli_args, settings);

  app
    // Add required resource plugins
    .add_plugin(SfxPlugin)
    // Editor plugins
    .add_plugins(EditorPlugins)
    .add_plugin(LevelSavePlugin)
    // Init Temple State
    .insert_resource(TempleState::edit_mode())
    .run();
}
