use clap::{Arg, App};

use crate::util::settings::GameFile;
use crate::level::LevelId;

pub const LOAD_ARG: &str = "load";

#[derive(Clone, Debug)]
pub struct CliArgs {
  pub load_level: Option<LevelId>
}

pub fn get_cli_args(version: String, game_file: &GameFile) -> CliArgs {
  let author_list = game_file.authors.clone().into_iter().reduce(|acc, auth| acc + auth.as_str()).unwrap();

  let cli = App::new(game_file.title.as_str())
                  .version(version.as_str())
                  .author(author_list.as_str())
                  .arg(Arg::with_name(LOAD_ARG)
                    .short("l")
                    .long("load")
                    .value_name("LEVEL_ID")
                    .help("Load into a specific level id"));

  let matches = cli.get_matches();

  let level_load_arg = if let Some(l) = matches.value_of(LOAD_ARG) {
    Some(l.parse::<u32>().expect("Supplied a non-valid input for Load Level Argument!"))
  } else {
    None
  };

  CliArgs {
    load_level: level_load_arg
  }
}
