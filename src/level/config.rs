//! Handles all config manifest loading for levels.
//!
//! Level manifests are loaded on game boot and
//! are stored in a Bevy Resource.

use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
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

/// File form of a [LevelMap], size optimized with a look up table for sprite
/// names.
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LevelMapFile {
  /// Look up table of sprite names to ids
  pub sprite_types: HashMap<u32, String>,
  /// Sprite entries for the level
  pub sprite_entries: Vec<LevelMapFileSpriteEntry>,
}

impl Into<LevelMap> for LevelMapFile {
  fn into(self) -> LevelMap {
    LevelMap {
      sprites: self
        .sprite_entries
        .iter()
        .map(|x| LevelMapSpriteEntry::new(self.sprite_types.get(&x.id).unwrap().clone(), x.pos))
        .collect(),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct LevelMapFileSpriteEntry {
  /// Position in the level (tile size independent)
  pub pos: IVec2,
  /// Sprite Id, as mapped within [LevelMapFile::sprite_types]
  pub id: u32,
}

impl LevelMapFileSpriteEntry {
  pub fn new(id: u32, pos: IVec2) -> Self {
    Self { pos, id }
  }
}

/// Map of [Level], stored as a binary file in `levelmaps/`
#[derive(Clone, Default, Debug)]
pub struct LevelMap {
  pub sprites: Vec<LevelMapSpriteEntry>,
}

impl Into<LevelMapFile> for LevelMap {
  fn into(self) -> LevelMapFile {
    let mut name_set: HashSet<String> = HashSet::new();
    // Generate types
    for entry in self.sprites.iter() {
      name_set.insert(entry.name.clone());
    }

    let sprite_types: HashMap<u32, String> = name_set
      .iter()
      .enumerate()
      .map(|(i, name)| (i as u32, name.clone()))
      .collect();
    let inverse_table: HashMap<String, u32> = name_set
      .iter()
      .enumerate()
      .map(|(i, name)| (name.clone(), i as u32))
      .collect();

    LevelMapFile {
      sprite_types,
      sprite_entries: self
        .sprites
        .iter()
        .map(|x| LevelMapFileSpriteEntry::new(inverse_table.get(&x.name).unwrap().clone(), x.pos))
        .collect(),
    }
  }
}

/// Sprite definitions for a level
#[derive(Clone, Debug)]
pub struct LevelMapSpriteEntry {
  /// Position in the level (tile size independent)
  pub pos: IVec2,
  /// Sprite Name, as defined in [LevelManifest]
  pub name: String,
}

impl LevelMapSpriteEntry {
  pub fn new(name: String, pos: IVec2) -> Self {
    Self { name, pos }
  }
}

// ==== In Engine Definitions (Consumes Manifests)

/// [LevelSpriteEntry] joined with position from [LevelMapSprite]
#[derive(Clone, Debug)]
pub struct HandledSprite {
  /// Sprite name and identifier.
  pub name: String,
  /// Position in a level
  pub pos: IVec2,
  /// Sprite offset within tile, defaults to [0, 0]
  pub offset: IVec2,
  /// Sprites texture
  pub texture: String,
  /// Sprites attributes
  pub attributes: Vec<String>,
}

impl HandledSprite {
  pub fn join_level_definitions(
    map_sprites: Vec<LevelMapSpriteEntry>,
    entries: Vec<LevelSpriteEntry>,
  ) -> Vec<HandledSprite> {
    let mut entries_map = HashMap::new();
    for entry in entries {
      entries_map.insert(entry.name.clone(), entry);
    }

    map_sprites
      .iter()
      .map(|map_sprite| {
        if let Some(sprite_entry) = entries_map.get(&map_sprite.name) {
          HandledSprite {
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

  pub fn decompose(list: Vec<Self>) -> (Vec<LevelMapSpriteEntry>, Vec<LevelSpriteEntry>) {
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

impl From<(LevelSpriteEntry, IVec2)> for HandledSprite {
  fn from((entry, pos): (LevelSpriteEntry, IVec2)) -> Self {
    Self {
      name: entry.name.clone(),
      pos,
      offset: entry.offset.clone(),
      texture: entry.texture.clone(),
      attributes: entry.attributes.clone(),
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
    let (map_sprites, level_sprites) = HandledSprite::decompose(self.sprites);
    let manifest = LevelManifest {
      name: self.name,
      music: self.music,
      sprites: level_sprites,
    };

    let map = LevelMap { sprites: map_sprites };

    (manifest, map)
  }
}
