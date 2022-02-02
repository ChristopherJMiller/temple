use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use bevy::asset::AssetPath;

use super::config::{HandledSprite, Level, LevelManifest, LevelMap, LevelMapFile};
use super::LevelId;
use crate::util::files::{from_game_root, LEVEL_DIR_PATH, LEVEL_MAP_DIR_PATH, SPRITE_TEXTURE_DIR_PATH};

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
    if let Ok(map) = rmp_serde::from_read_ref::<_, LevelMapFile>(&file) {
      Some(map.into())
    } else {
      None
    }
  } else {
    None
  }
}

pub fn prepare_level_from_manifests(manifest: LevelManifest, map: LevelMap) -> Level {
  let handled_sprites = HandledSprite::join_level_definitions(map.sprites, manifest.sprites.clone());
  Level::from((manifest, handled_sprites))
}

pub fn get_texture_path<'a>(texture_local_path: &String) -> AssetPath<'a> {
  from_game_root(Path::new(SPRITE_TEXTURE_DIR_PATH).join(texture_local_path.as_str())).into()
}

pub fn levels_have_same_music(a: LevelId, b: LevelId) -> bool {
  if let Some(a) = get_manifest_by_id(a) {
    if let Some(b) = get_manifest_by_id(b) {
      return a.music.eq(&b.music);
    }
  }

  return false;
}
