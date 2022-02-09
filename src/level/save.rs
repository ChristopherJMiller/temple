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
    if let Err(err) = fs::write(manifest_path.clone(), manifest_contents) {
      if cfg!(not(test)) {
        error!(target: "save_loaded_level", "Was unable to save the level manifest! {}", err.to_string());
      } else {
        panic!("Cannot write to {:?}: {}", manifest_path, err.to_string());
      }
    }

    // Save map
    let map_path = get_level_map_path_from_id(level_id);
    let map_contents = rmp_serde::to_vec::<LevelMapFile>(&map.into()).unwrap();
    if let Err(err) = fs::write(map_path, map_contents) {
      if cfg!(not(test)) {
        error!(target: "save_loaded_level", "Was unable to save the level map!, {}", err.to_string());
      } else {
        panic!("{}", err.to_string());
      }
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

#[cfg(test)]
mod tests {
  use std::fs;

  use crate::level::config::*;
  use crate::level::load::*;
  use crate::level::save::*;
  use crate::level::util::*;
  use crate::util::files::*;

  #[test]
  fn test_save_loaded_level() {
    const NAME: &str = "test level";
    let mut world = World::default();

    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(save_loaded_level);

    world.spawn().insert_bundle((
      PreparedLevel(Level {
        name: NAME.to_string(),
        music: "".to_string(),
        sprites: vec![HandledSprite::new("sprite", (0, 0), (0, 0), "", vec!["solid"])],
      }),
      LoadLevel(0),
      LevelLoadComplete,
      SaveLevel,
    ));

    // Bootstrap dirs
    fs::remove_dir_all(from_game_root(ASSET_PATH));
    fs::create_dir_all(from_game_root(LEVEL_DIR_PATH)).unwrap();
    fs::create_dir_all(from_game_root(LEVEL_MAP_DIR_PATH)).unwrap();

    update_stage.run(&mut world);
    assert_eq!(world.query::<&SaveLevel>().iter(&world).len(), 0);

    let level_manifest = get_manifest_by_id(0).expect("Failed to get level manifest id 0");
    assert_eq!(NAME, level_manifest.name.as_str());
  }
}
