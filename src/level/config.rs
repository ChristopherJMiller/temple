use std::fs;
use std::path::Path;
use std::vec::Vec;

use bevy::prelude::*;
use png::{BitDepth, ColorType, Decoder};
use serde::Deserialize;

use super::{LevelId, LevelMap};
use crate::sprite::SpriteMap;
use crate::util::files::LEVEL_FILE_PATH;

pub struct LevelFileVersion(pub u32);

/// Object of levels.toml
#[derive(Deserialize)]
pub struct LevelFile {
  pub version: u32,
  pub levels: Vec<LevelDefinition>,
}

/// Single level in levels.toml
#[derive(Deserialize)]
pub struct LevelDefinition {
  /// Id, must be unique
  pub id: LevelId,
  /// Path to level map
  pub sprite_map: String,
}

/// Sprite definition for a level
pub struct LevelSprite {
  pub pos: UVec2,
  pub id: u32,
}

/// Stored object of a level, stores redundant id and list of sprites in the
/// level
pub struct Level {
  #[allow(dead_code)]
  pub id: LevelId,
  pub sprites: Vec<LevelSprite>,
}

/// Loads all levels into the [LevelMap] resource
pub fn load_level_files(version: Res<LevelFileVersion>, sprites: Res<SpriteMap>, mut levels: ResMut<LevelMap>) {
  let version_num = version.0;

  // Load LevelFile
  if let Ok(file) = fs::read_to_string(LEVEL_FILE_PATH) {
    // Deserialize
    match toml::from_str::<LevelFile>(file.as_str()) {
      // Check for valid version
      Ok(level_list) => {
        if level_list.version != version_num {
          panic!(
            "Incorrect file version, should be {} but found {}",
            version_num, level_list.version
          );
        }

        // Load level sprite definitions into map
        for level in level_list.levels.iter() {
          // Load level map
          let full_path = Path::new("assets/textures").join(level.sprite_map.as_str());
          if let Ok(bitmap) = fs::File::open(full_path) {
            let decoder = Decoder::new(bitmap);
            if let Ok((info, mut reader)) = decoder.read_info() {
              if info.color_type != ColorType::RGBA {
                panic!("Bitmap {} incorrect color type, should be RGBA", level.sprite_map);
              }

              if info.bit_depth != BitDepth::Eight {
                panic!("Bitmap {} incorrect bit depth, should be eight", level.sprite_map);
              }

              let mut buf = vec![0; info.buffer_size()];
              reader.next_frame(&mut buf).unwrap();

              let mut level_sprites: Vec<LevelSprite> = Vec::new();

              // Load each level sprite
              for i in (0..info.buffer_size()).step_by(4) {
                // Sprites should not use transparency
                if buf[i + 3] != 255 {
                  continue;
                }

                let level_x = (i / 4) as u32 % info.width;
                let level_y = info.height - ((i / 4) as u32 / info.width);

                let tile_r: u32 = buf[i] as u32;
                let tile_g: u32 = buf[i + 1] as u32;
                let tile_b: u32 = buf[i + 2] as u32;

                // Build sprite id using RGB value
                let sprite_id: u32 = (tile_r << 8) | (tile_g << 4) | tile_b;

                if !sprites.contains_key(&sprite_id) {
                  panic!("Attempted to register level with invalid sprite id {}", sprite_id);
                }

                level_sprites.push(LevelSprite {
                  id: sprite_id,
                  pos: UVec2::new(level_x, level_y),
                });
              }

              let level_obj = Level {
                id: level.id,
                sprites: level_sprites,
              };

              if levels.insert(level.id, level_obj).is_some() {
                panic!("Conflicting level definitions for id {}", level.id);
              }
            } else {
              panic!("Failed to read PNG file {}", level.sprite_map);
            }
          } else {
            panic!("Failed to find bitmap {} for level id {}", level.sprite_map, level.id);
          }
        }

        info!(target: "load_level_files", "{} levels registered", level_list.levels.len());
      },
      Err(err) => {
        panic!("Failed to parse sprite types file: {}", err);
      },
    }
  } else {
    panic!("Unable to load sprite types file!");
  }
}
