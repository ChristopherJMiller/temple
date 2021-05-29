use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::vec::Vec;

use bevy::prelude::*;
use serde::Deserialize;

struct SpriteFileVersion(u32);

#[derive(Deserialize)]
struct SpriteTypesFile {
  version: u32,
  types: Vec<SpriteType>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct SpriteType {
  id: String,
  attributes: Vec<String>,
}

#[derive(Deserialize)]
struct SpriteFile {
  version: u32,
  sprites: Vec<SpriteEntry>,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct SpriteEntry {
  name: String,
  color: u32, // 24-bit RGB
  sprite_type: String,
  offset_x: u32,
  offset_y: u32,
  texture: String,
}

#[derive(Debug, Clone, Default)]
pub struct Sprite {
  id: u32,
  name: String,
  offset_x: u32,
  offset_y: u32,
  texture: Handle<ColorMaterial>,
  attributes: Vec<String>,
}

fn load_sprite_types(version: Res<SpriteFileVersion>, mut sprite_types: ResMut<HashMap<String, SpriteType>>) {
  let version_num = version.0;

  if let Ok(file) = fs::read_to_string("assets/sprites/types.toml") {
    match toml::from_str::<SpriteTypesFile>(file.as_str()) {
      Ok(types) => {
        if types.version != version_num {
          panic!(
            "Incorrect file version, should be {} but found {}",
            version_num, types.version
          );
        }

        for sprite_type in types.types.iter() {
          if let Some(_) = sprite_types.insert(sprite_type.id.clone(), sprite_type.clone()) {
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

fn load_sprites(
  version: Res<SpriteFileVersion>,
  mut sprite_map: ResMut<HashMap<u32, Sprite>>,
  sprite_types: Res<HashMap<String, SpriteType>>,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let version_num = version.0;

  if let Ok(file) = fs::read_to_string("assets/sprites/sprites.toml") {
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
            let texture_handle = asset_server.load(full_path.to_str().unwrap());

            let full_sprite = Sprite {
              id: sprite.color,
              name: sprite.name.clone(),
              attributes: sprite_type.attributes.clone(),
              offset_x: sprite.offset_x,
              offset_y: sprite.offset_y,
              texture: materials.add(texture_handle.into()),
            };

            if let Some(_) = sprite_map.insert(sprite.color, full_sprite) {
              panic!("Conflicting type definitions for color id {}", sprite.color);
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

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource::<SpriteFileVersion>(SpriteFileVersion(1))
      .init_resource::<HashMap<String, SpriteType>>()
      .init_resource::<HashMap<u32, Sprite>>()
      .add_startup_system(load_sprite_types.system().label(SpritePluginSteps::LoadSpriteTypes))
      .add_startup_system(
        load_sprites
          .system()
          .label(SpritePluginSteps::LoadSprites)
          .after(SpritePluginSteps::LoadSpriteTypes),
      );
  }
}
