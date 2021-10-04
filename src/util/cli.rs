use clap::{App, Arg};
use bevy::prelude::*;

use crate::level::LevelId;
use crate::level::load::LoadLevel;
use crate::util::settings::GameFile;

pub const LOAD_ARG: &str = "load";
const TEMPLE_URL: &str = "chrismiller.xyz";

#[derive(Clone, Debug)]
pub struct CliArgs {
  pub load_level: Option<LevelId>,
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
    );

  let matches = cli.get_matches();

  let level_load_arg = if let Some(l) = matches.value_of(LOAD_ARG) {
    Some(
      l.parse::<u32>()
        .expect("Supplied a non-valid input for Load Level Argument!"),
    )
  } else {
    None
  };

  CliArgs {
    load_level: level_load_arg,
  }
}

/// Consumes incoming CLI arguments within Bevy
pub fn handle_cli_args(mut commands: Commands, cli_args: Res<CliArgs>) {
  // --load <level>
  if let Some(level) = cli_args.load_level {
    commands.spawn().insert(LoadLevel(level));
  }
}
