//! Sprite and sprite type config management

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::vec::Vec;

use bevy::prelude::*;
use serde::Deserialize;

use crate::util::files::{SPRITE_FILE_PATH, SPRITE_TYPE_FILE_PATH};

/// Constant sprite size
pub const SPRITE_SIZE: u32 = 16;

pub type SpriteId = String;
pub type SpriteTypeMap = HashMap<String, SpriteType>;
pub type SpriteMap = HashMap<SpriteId, GameSprite>;

struct SpriteFileVersion(u32);

/// Object of sprites/types.toml
#[derive(Deserialize)]
pub struct SpriteTypesFile {
  version: u32,
  types: Vec<SpriteType>,
}

/// Type Object within sprites/types.toml
#[derive(Deserialize, Debug, Clone, Default)]
pub struct SpriteType {
  id: String,
  attributes: Vec<String>,
}

/// Object of sprites/sprites.toml
#[derive(Deserialize)]
pub struct SpriteFile {
  version: u32,
  sprites: Vec<SpriteEntry>,
}

/// Sprite Object within sprites/sprites.toml
#[derive(Deserialize, Debug, Clone, Default)]
struct SpriteEntry {
  name: String,
  sprite_type: String,
  offset_x: u32,
  offset_y: u32,
  texture: String,
}

/// Sprite definition to be used in game
#[derive(Debug, Clone, Default)]
pub struct GameSprite {
  pub name: String,
  pub offset_x: u32,
  pub offset_y: u32,
  pub texture: Handle<ColorMaterial>,
  pub attributes: Vec<String>,
}

/// Loads all sprite types into memory map
fn load_sprite_types(version: Res<SpriteFileVersion>, mut sprite_types: ResMut<SpriteTypeMap>) {
  let version_num = version.0;

  if let Ok(file) = fs::read_to_string(SPRITE_TYPE_FILE_PATH) {
    match toml::from_str::<SpriteTypesFile>(file.as_str()) {
      Ok(types) => {
        // Ensure is same version number as sprites definition file
        if types.version != version_num {
          panic!(
            "Incorrect file version, should be {} but found {}",
            version_num, types.version
          );
        }

        for sprite_type in types.types.iter() {
          if sprite_types
            .insert(sprite_type.id.clone(), sprite_type.clone())
            .is_some()
          {
            panic!("Conflicting type definitions for id {}", sprite_type.id);
          }
        }

        info!(target: "load_sprite_types", "{} sprite types registered", types.types.len());
      },
      Err(err) => {
        panic!("Failed to parse sprite types file: {}", err);
      },
    }
  } else {
    panic!("Unable to load sprite types file!");
  }
}

/// Loads all sprites and sprite textures into memory
fn load_sprites(
  version: Res<SpriteFileVersion>,
  sprite_types: Res<SpriteTypeMap>,
  asset_server: Res<AssetServer>,
  mut sprite_map: ResMut<SpriteMap>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let version_num = version.0;

  // Load Sprite File
  if let Ok(file) = fs::read_to_string(SPRITE_FILE_PATH) {
    match toml::from_str::<SpriteFile>(file.as_str()) {
      Ok(sprites) => {
        if sprites.version != version_num {
          panic!(
            "Incorrect file version, should be {} but found {}",
            version_num, sprites.version
          );
        }

        for sprite in sprites.sprites.iter() {
          if let Some(sprite_type) = sprite_types.get(&sprite.sprite_type) {
            let full_path = Path::new("textures").join(sprite.texture.as_str());

            if !Path::new("assets").join(full_path.clone()).is_file() {
              panic!(
                "File not found when registering {}: {}",
                sprite.name,
                full_path.to_str().unwrap()
              );
            }

            // Load all sprite textures into bevy AssetServer
            let texture_handle = asset_server.load(full_path.to_str().unwrap());

            // Build Sprite definition
            let full_sprite = GameSprite {
              name: sprite.name.clone(),
              attributes: sprite_type.attributes.clone(),
              offset_x: sprite.offset_x,
              offset_y: sprite.offset_y,
              texture: materials.add(texture_handle.into()),
            };

            if sprite_map.insert(sprite.name.clone(), full_sprite).is_some() {
              panic!("Conflicting type definitions for sprite id {}", sprite.name);
            }
          } else {
            panic!(
              "Attempted to register sprite with unknown sprite type {}",
              sprite.sprite_type
            );
          }
        }

        info!(target: "load_sprites", "{} sprites registered", sprites.sprites.len());
      },
      Err(err) => {
        panic!("Failed to parse sprite file: {}", err);
      },
    }
  } else {
    panic!("Unable to load sprites file!");
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SpritePluginSteps {
  LoadSpriteTypes,
  LoadSprites,
}

/// SpritePlugin for handling loading sprite config files (attribute and sprite
/// definitions)
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource::<SpriteFileVersion>(SpriteFileVersion(1))
      .init_resource::<SpriteTypeMap>()
      .init_resource::<SpriteMap>()
      .add_startup_system(load_sprite_types.system().label(SpritePluginSteps::LoadSpriteTypes))
      .add_startup_system(
        load_sprites
          .system()
          .label(SpritePluginSteps::LoadSprites)
          .after(SpritePluginSteps::LoadSpriteTypes),
      );
  }
}
