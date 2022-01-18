use std::collections::HashMap;
use std::fs;

use bevy::math::IVec2;

use super::ui::LevelMenuItem;
use crate::level::config::LevelSpriteEntry;
use crate::level::util::{get_level_manifests, get_manifest_by_id, get_map_by_id};
use crate::level::LevelId;
use crate::util::files::{from_game_root, MUSIC_DIR_PATH, SPRITE_TEXTURE_DIR_PATH};

#[derive(Clone)]
pub struct AddSpriteForm {
  /// Sprite name and identifier.
  pub name: String,
  pub offset: [String; 2],
  /// Sprites texture
  pub texture: String,
  /// Sprites attributes
  pub attributes: Vec<String>,
}

impl Default for AddSpriteForm {
  fn default() -> Self {
    Self {
      name: Default::default(),
      offset: ["0".to_string(), "0".to_string()],
      texture: Default::default(),
      attributes: Default::default(),
    }
  }
}

impl Into<LevelSpriteEntry> for AddSpriteForm {
  fn into(self) -> LevelSpriteEntry {
    let offsets: Vec<_> = self.offset.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    LevelSpriteEntry {
      name: self.name,
      offset: IVec2::new(offsets[0], offsets[1]),
      texture: self.texture,
      attributes: self.attributes,
    }
  }
}

pub fn format_menu_item(item: &LevelMenuItem) -> String {
  format!("ID {} | {}", item.0, item.1.name.clone())
}

pub fn get_level_menu_items() -> Vec<LevelMenuItem> {
  get_level_manifests().iter().map(|x| x.clone().into()).collect()
}

fn get_asset_dir_files(path: &str) -> Vec<String> {
  let dir = fs::read_dir(from_game_root(path)).unwrap();
  dir
    .into_iter()
    .filter(|x| !x.as_ref().unwrap().path().is_dir())
    .map(|x| x.unwrap())
    .map(|x| x.file_name().to_str().unwrap().to_string())
    .collect()
}

pub fn get_music_files() -> Vec<String> {
  get_asset_dir_files(MUSIC_DIR_PATH)
}

pub fn get_sprite_texture_files() -> Vec<String> {
  get_asset_dir_files(SPRITE_TEXTURE_DIR_PATH)
}

pub fn validate_add_sprite_form(form: &AddSpriteForm) -> bool {
  !form.name.is_empty()
    && !form.texture.is_empty()
    && !form.offset[0].is_empty()
    && !form.offset[1].is_empty()
    && form.offset[0].parse::<i32>().is_ok()
    && form.offset[1].parse::<i32>().is_ok()
}

pub fn load_level_sprite_entries(id: LevelId) -> Option<Vec<LevelSpriteEntry>> {
  if let Some(manifest) = get_manifest_by_id(id) {
    Some(manifest.sprites)
  } else {
    None
  }
}

pub fn get_sprite_table(id: LevelId) -> Option<HashMap<IVec2, String>> {
  if let Some(map) = get_map_by_id(id) {
    let mut result: HashMap<IVec2, String> = HashMap::new();
    for sprite in map.sprites.iter() {
      result.insert(sprite.pos, sprite.name.clone());
    }

    Some(result)
  } else {
    None
  }
}
