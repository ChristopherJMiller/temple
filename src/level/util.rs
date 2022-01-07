use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use bevy::prelude::*;

use super::config::{HandledSprite, JoinedLevelSpriteEntry, Level, LevelManifest, LevelMap};
use super::LevelId;
use crate::util::files::{from_game_root, LEVEL_DIR_PATH, LEVEL_MAP_DIR_PATH};

pub fn get_level_manifests() -> Vec<(LevelId, LevelManifest)> {
  let level_manifests = fs::read_dir(from_game_root(LEVEL_DIR_PATH)).expect("Unable to load level manifest directory.");
  level_manifests
    .into_iter()
    .filter(|x| !x.as_ref().unwrap().path().is_dir())
    .map(|x| x.unwrap())
    .map(|x| {
      (
        get_level_id_from_path(&x.path()).unwrap(),
        fs::read_to_string(x.path()).unwrap(),
      )
    })
    .map(|(id, file)| (id, toml::from_str::<LevelManifest>(&file).unwrap()))
    .collect()
}

pub fn get_level_id_from_path(path: &Path) -> Result<LevelId, <LevelId as FromStr>::Err> {
  Path::with_extension(path, "")
    .file_name()
    .unwrap()
    .to_str()
    .unwrap()
    .parse::<LevelId>()
}

pub fn get_level_manifest_path_from_id(id: LevelId) -> PathBuf {
  Path::new(&from_game_root(LEVEL_DIR_PATH))
    .join(id.to_string())
    .with_extension("toml")
}

pub fn get_level_map_path_from_id(id: LevelId) -> PathBuf {
  Path::new(&from_game_root(LEVEL_MAP_DIR_PATH))
    .join(id.to_string())
    .with_extension("levelmap")
}

pub fn get_manifest_by_id(id: LevelId) -> Option<LevelManifest> {
  let path = get_level_manifest_path_from_id(id);
  if let Ok(file) = fs::read_to_string(path) {
    if let Ok(manifest) = toml::from_str::<LevelManifest>(&file) {
      Some(manifest)
    } else {
      None
    }
  } else {
    None
  }
}

pub fn get_map_by_id(id: LevelId) -> Option<LevelMap> {
  let path = get_level_map_path_from_id(id);
  if let Ok(file) = fs::read(path) {
    if let Ok(map) = rmp_serde::from_read_ref::<_, LevelMap>(&file) {
      Some(map)
    } else {
      None
    }
  } else {
    None
  }
}

pub fn prepare_level_from_manifests(asset_server: &Res<AssetServer>, manifest: LevelManifest, map: LevelMap) -> Level {
  let joined_entries = JoinedLevelSpriteEntry::join_level_definitions(map.sprites, manifest.sprites.clone());
  let handled_sprites: Vec<_> = joined_entries
    .iter()
    .map(|entry| HandledSprite::from_joined_entry(entry, asset_server))
    .collect();
  Level::from((manifest, handled_sprites))
}
