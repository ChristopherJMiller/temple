//! Handles all config manifest loading for levels.
//!
//! Level manifests are loaded on game boot and
//! are stored in a Bevy Resource.

use std::collections::HashMap;
use std::vec::Vec;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::util::load_sprite_texture;

// ==== Asset File Definitions

/// Conversion of game unit to pixel.
pub const SPRITE_SIZE: u32 = 16;

/// Structure of a level toml file.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LevelManifest {
  /// Level Name
  pub name: String,
  /// Background Music for Level
  pub music: String,
  /// Sprites used in Level
  pub sprites: Vec<LevelSpriteEntry>,
}

/// Sprites used in a [Level], defined in a [LevelManifest]
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct LevelSpriteEntry {
  /// Sprite name and identifier.
  pub name: String,
  /// Sprite offset within tile, defaults to [0, 0]
  #[serde(default)]
  pub offset: IVec2,
  /// Sprites texture
  pub texture: String,
  /// Sprites attributes
  pub attributes: Vec<String>,
}

/// Map of [Level], stored as a binary file in `levelmaps/`
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LevelMap {
  pub sprites: Vec<LevelMapSpriteEntry>,
}

/// Sprite definitions for a level
#[derive(Serialize, Deserialize, Clone)]
pub struct LevelMapSpriteEntry {
  /// Position in the level
  pub pos: UVec2,
  /// Sprite Name, as defined in [LevelManifest]
  pub name: String,
}

// ==== In Engine Definitions (Consumes Manifests)

/// [LevelSpriteEntry] joined with position from [LevelMapSprite]
#[derive(Clone)]
pub struct JoinedLevelSpriteEntry {
  /// Sprite name and identifier.
  pub name: String,
  /// Position in a level
  pub pos: UVec2,
  /// Sprite offset within tile, defaults to [0, 0]
  pub offset: IVec2,
  /// Sprites texture
  pub texture: String,
  /// Sprites attributes
  pub attributes: Vec<String>,
}

impl JoinedLevelSpriteEntry {
  pub fn join_level_definitions(
    map_sprites: Vec<LevelMapSpriteEntry>,
    entries: Vec<LevelSpriteEntry>,
  ) -> Vec<JoinedLevelSpriteEntry> {
    let mut entries_map = HashMap::new();
    for entry in entries {
      entries_map.insert(entry.name.clone(), entry);
    }

    map_sprites
      .iter()
      .map(|map_sprite| {
        if let Some(sprite_entry) = entries_map.get(&map_sprite.name) {
          JoinedLevelSpriteEntry {
            name: sprite_entry.name.clone(),
            pos: map_sprite.pos,
            offset: sprite_entry.offset,
            texture: sprite_entry.texture.clone(),
            attributes: sprite_entry.attributes.clone(),
          }
        } else {
          panic!("Could not find sprite entry for {}", map_sprite.name);
        }
      })
      .collect()
  }

  pub fn decompose(list: Vec<JoinedLevelSpriteEntry>) -> (Vec<LevelMapSpriteEntry>, Vec<LevelSpriteEntry>) {
    let mut map_sprite_entries: Vec<LevelMapSpriteEntry> = Vec::default();
    let mut level_sprites: HashMap<String, LevelSpriteEntry> = HashMap::default();
    for entry in list {
      if !level_sprites.contains_key(&entry.name) {
        level_sprites.insert(
          entry.name.clone(),
          LevelSpriteEntry {
            name: entry.name.clone(),
            offset: entry.offset,
            texture: entry.texture,
            attributes: entry.attributes,
          },
        );
      }

      map_sprite_entries.push(LevelMapSpriteEntry {
        name: entry.name,
        pos: entry.pos,
      });
    }

    (map_sprite_entries, level_sprites.values().cloned().collect())
  }
}

/// Sprite with loaded texture, to be used in game.
#[derive(Debug, Clone, Default)]
pub struct HandledSprite {
  pub name: String,
  pub pos: UVec2,
  pub offset: IVec2,
  pub texture_path: String,
  pub texture: Handle<ColorMaterial>,
  pub attributes: Vec<String>,
}

impl HandledSprite {
  pub fn from_joined_entry(
    entry: &JoinedLevelSpriteEntry,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
  ) -> Self {
    Self {
      name: entry.name.clone(),
      pos: entry.pos,
      offset: entry.offset,
      texture_path: entry.texture.clone(),
      texture: load_sprite_texture(asset_server, materials, &entry.texture),
      attributes: entry.attributes.clone(),
    }
  }
}

impl Into<JoinedLevelSpriteEntry> for &HandledSprite {
  fn into(self) -> JoinedLevelSpriteEntry {
    JoinedLevelSpriteEntry {
      name: self.name.clone(),
      pos: self.pos,
      offset: self.offset,
      texture: self.texture_path.clone(),
      attributes: self.attributes.clone(),
    }
  }
}

/// Stored object of a level, stores associated info and list of sprites used in
/// level
#[derive(Debug, Clone)]
pub struct Level {
  /// Level Name
  pub name: String,
  /// Music track for the level (will be loaded by the asset server).
  pub music: String,
  /// Sprites for Level Map
  pub sprites: Vec<HandledSprite>,
}

/// Form Level from Manifest Components
impl From<(LevelManifest, Vec<HandledSprite>)> for Level {
  fn from((manifest, map): (LevelManifest, Vec<HandledSprite>)) -> Self {
    Self {
      name: manifest.name,
      music: manifest.music,
      sprites: map,
    }
  }
}

/// Decompose Level into Manifest Forms
impl Into<(LevelManifest, LevelMap)> for Level {
  fn into(self) -> (LevelManifest, LevelMap) {
    let joined_entries: Vec<JoinedLevelSpriteEntry> = self.sprites.iter().map(|x| x.into()).collect();
    let (map_sprites, level_sprites) = JoinedLevelSpriteEntry::decompose(joined_entries);
    let manifest = LevelManifest {
      name: self.name,
      music: self.music,
      sprites: level_sprites,
    };

    let map = LevelMap { sprites: map_sprites };

    (manifest, map)
  }
}
