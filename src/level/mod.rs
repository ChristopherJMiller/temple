//! Level config management and level loading.

use bevy::prelude::*;
use load::*;
use verify::verify_level_files;

use self::load::{next_checkpoint, wait_until_unloaded};
use self::next::auto_next_level;
use self::util::get_level_manifests;
use crate::game::attributes::{Attribute, Goal};

pub mod config;
pub mod load;
pub mod next;
pub mod save;
pub mod util;
pub mod verify;

pub type LevelId = u32;

/// [Plugin] for level management systems
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<TotalExits>()
      .add_startup_system(verify_level_files)
      .add_startup_system(count_exits)
      .add_system(wait_until_unloaded)
      .add_system(prepare_level)
      .add_system(load_level)
      .add_system(unload_level)
      .add_system(apply_save_on_load)
      .add_system(transition_level)
      .add_system(auto_next_level)
      .add_system(next_checkpoint);
  }
}

#[derive(Default)]
pub struct TotalExits(pub usize);

fn count_exits(mut exits: ResMut<TotalExits>) {
  let levels = get_level_manifests();
  let exit_count = levels.iter().fold(0, |acc, (_, manifest)| {
    let level_exit_count = manifest.sprites.iter().fold(
      0,
      |acc, item| {
        if item.name.as_str() == Goal::KEY {
          acc + 1
        } else {
          acc
        }
      },
    );

    acc + level_exit_count
  });

  exits.0 = exit_count;
}
