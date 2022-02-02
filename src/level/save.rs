use std::fs;

use bevy::prelude::*;

use super::load::{LevelLoadComplete, LoadLevel, PreparedLevel};
use super::util::{get_level_manifest_path_from_id, get_level_map_path_from_id};
use crate::level::config::LevelMapFile;

/// Command to save loaded level
#[derive(Component)]
pub struct SaveLevel;

pub fn save_loaded_level(
  mut commands: Commands,
  query: Query<(Entity, &PreparedLevel, &LoadLevel), (With<SaveLevel>, With<LevelLoadComplete>)>,
) {
  query.for_each(|(e, prepared_level, load_level)| {
    let (manifest, map) = prepared_level.0.clone().into();
    let level_id = load_level.0;

    // Save Manifest
    let manifest_path = get_level_manifest_path_from_id(level_id);
    let manifest_contents = toml::to_string_pretty(&manifest).unwrap();
    if fs::write(manifest_path, manifest_contents).is_err() {
      warn!(target: "save_loaded_level", "Was unable to save the level manifest!");
    }

    // Save map
    let map_path = get_level_map_path_from_id(level_id);
    let map_contents = rmp_serde::to_vec::<LevelMapFile>(&map.into()).unwrap();
    if fs::write(map_path, map_contents).is_err() {
      warn!(target: "save_loaded_level", "Was unable to save the level map!");
    }

    commands.entity(e).remove::<SaveLevel>();

    info!(target: "save_loaded_level", "Saved Level Successfully!");
  });
}

/// [Plugin] for saving levels, to be used in edit mode.
pub struct LevelSavePlugin;

impl Plugin for LevelSavePlugin {
  fn build(&self, app: &mut App) {
    app.add_system(save_loaded_level);
  }
}
