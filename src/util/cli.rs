use clap::{Arg, App, ArgMatches};

use crate::util::settings::GameFile;

pub fn get_cli_matches(version: String, game_file: &GameFile) -> ArgMatches {
  let author_list = game_file.authors.clone().into_iter().reduce(|acc, auth| acc + auth.as_str()).unwrap();

  let cli = App::new(game_file.title.as_str())
                  .version(version.as_str())
                  .author(author_list.as_str())
                  .arg(Arg::with_name("load")
                    .short("l")
                    .long("load")
                    .value_name("LEVEL_ID")
                    .help("Load into a specific level id"));

  cli.get_matches()
}
