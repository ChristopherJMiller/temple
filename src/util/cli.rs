//! Handles the CLI options

use bevy::prelude::*;
use clap::{App, Arg};

use crate::level::load::LoadLevel;
use crate::level::LevelId;
use crate::state::game_state::{TempleState, GameMode};
use crate::ui::LoadTitleScreen;
use crate::util::settings::GameFile;

/// `load` argument.
pub const LOAD_ARG: &str = "load";

/// `fps` argument.
pub const FPS_ARG: &str = "fps";

/// `editor` argument.
pub const EDITOR_ARG: &str = "editor";

/// Website about the Temple project
const TEMPLE_URL: &str = "https://github.com/ChristopherJMiller/temple";

/// Output of the CLI processing.
/// Contains all possible argument flags and their
/// supplied values.
#[derive(Clone, Debug)]
pub struct CliArgs {
  pub load_level: Option<LevelId>,
  pub show_fps_counter: bool,
  pub edit_mode: bool,
}

impl CliArgs {
  pub fn builder() -> CliArgsBuilder {
    CliArgsBuilder::default()
  }
}

#[derive(Default)]
pub struct CliArgsBuilder {
  pub load_level: Option<LevelId>,
  pub show_fps_counter: bool,
  pub edit_mode: bool,
}

impl CliArgsBuilder {
  pub fn load_level(mut self, id: LevelId) -> Self {
    self.load_level = Some(id);
    self
  }

  pub fn show_fps_counter(mut self) -> Self {
    self.show_fps_counter = true;
    self
  }

  pub fn enable_editor(mut self) -> Self {
    self.edit_mode = true;
    self
  }

  pub fn build(self) -> CliArgs {
    CliArgs {
      load_level: self.load_level,
      show_fps_counter: self.show_fps_counter,
      edit_mode: self.edit_mode,
    }
  }
}

/// Command line tool using [clap](https://docs.rs/clap/2.33.3/clap/)
pub fn get_cli_args(version: String, game_file: &GameFile) -> CliArgs {
  let author_list = game_file
    .authors
    .clone()
    .into_iter()
    .reduce(|acc, auth| acc + auth.as_str())
    .unwrap();
  let about = format!("A game built on the Temple Platform ({})", TEMPLE_URL);

  let cli = App::new(game_file.title.as_str())
    .version(version.as_str())
    .author(author_list.as_str())
    .about(about.as_str())
    .arg(
      Arg::with_name(LOAD_ARG)
        .short("l")
        .long("load")
        .value_name("LEVEL_ID")
        .help("Load into a specific level id"),
    )
    .arg(
      Arg::with_name(FPS_ARG)
        .long("fps")
        .takes_value(false)
        .help("Enables the in-game fps counter"),
    )
    .arg(Arg::with_name(EDITOR_ARG).long("editor").help("Enters the editor"));

  let matches = cli.get_matches();

  let mut builder = CliArgs::builder();

  if let Some(l) = matches.value_of(LOAD_ARG) {
    if let Ok(id) = l.parse::<u32>() {
      builder = builder.load_level(id);
    } else {
      panic!(
        "Invalid entry for the load level argument. Expected an integer, found \"{}\"",
        l
      );
    }
  }

  if matches.is_present(FPS_ARG) {
    builder = builder.show_fps_counter();
  }

  if matches.is_present(EDITOR_ARG) {
    builder = builder.enable_editor();
  }

  builder.build()
}

/// Consumes incoming CLI arguments within Bevy
pub fn handle_cli_args(mut commands: Commands, mut temple_state: ResMut<TempleState>, cli_args: Res<CliArgs>) {
  // Command line cli is for play mode only
  if !temple_state.in_edit_mode() {
    // --load <level>
    if let Some(level) = cli_args.load_level {
      commands.spawn().insert(LoadLevel(level));
      temple_state.game_mode = GameMode::InLevel(level);
    } else {
      commands.spawn().insert(LoadTitleScreen);
    }
  }
}
