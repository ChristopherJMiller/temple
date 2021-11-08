//! Handles the CLI options

use bevy::prelude::*;
use clap::{App, Arg};

use crate::level::load::LoadLevel;
use crate::level::LevelId;
use crate::util::settings::GameFile;
use crate::ui::LoadTitleScreen;


/// `load` argument.
pub const LOAD_ARG: &str = "load";

/// `fps` argument.
pub const FPS_ARG: &str = "fps";

/// Website about the Temple project
const TEMPLE_URL: &str = "https://github.com/ChristopherJMiller/temple";

/// Output of the CLI processing.
/// Contains all possible argument flags and their
/// supplied values.
#[derive(Clone, Debug)]
pub struct CliArgs {
  pub load_level: Option<LevelId>,
  pub show_fps_counter: bool,
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

  pub fn build(self) -> CliArgs {
    CliArgs {
      load_level: self.load_level,
      show_fps_counter: self.show_fps_counter
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
        .help("Load into a specific level id")
    )
    .arg(
      Arg::with_name(FPS_ARG)
      .long("fps")
      .takes_value(false)
      .help("Enables the in-game fps counter")
    );

  let matches = cli.get_matches();

  let mut builder = CliArgs::builder();

  if let Some(l) = matches.value_of(LOAD_ARG) {
    if let Ok(id) = l.parse::<u32>() {
      builder = builder.load_level(id);
    } else {
      panic!("Invalid entry for the load level argument. Expected an integer, found \"{}\"", l);
    }
  }

  if matches.is_present(FPS_ARG) {
    builder = builder.show_fps_counter();
  }

  builder.build()
}

/// Consumes incoming CLI arguments within Bevy
pub fn handle_cli_args(mut commands: Commands, cli_args: Res<CliArgs>) {
  // --load <level>
  if let Some(level) = cli_args.load_level {
    commands.spawn().insert(LoadLevel(level));
  } else {
    commands.spawn().insert(LoadTitleScreen);
  }
}
