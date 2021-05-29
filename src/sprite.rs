use bevy::prelude::*;
use std::fs;
use std::vec::Vec;
use std::collections::HashMap;
use serde::Deserialize;
use toml::de::Error;

struct SpriteFileVersion(u32);

#[derive(Deserialize)]
struct SpriteTypesFile {
  version: u32,
  types: Vec<SpriteType>
}

#[derive(Deserialize, Debug, Clone, Default)]
struct SpriteType {
  id: String,
  attributes: Vec<String>
}

struct Sprite {
  name: String,
  color: u32, // 24-bit RGB
  texture: String, // asset path to 16x16 texture
}

fn load_sprite_types(version: Res<SpriteFileVersion>, mut sprite_types: ResMut<HashMap<String, SpriteType>>) {
  let version_num = version.0;

  if let Ok(file) = fs::read_to_string("assets/sprites/types.toml") {    
    match toml::from_str::<SpriteTypesFile>(file.as_str()) {
      Ok(types) => {
        if types.version != version_num {
          panic!("Incorrect file version, should be {} but found {}", version_num, types.version);
        }

        for sprite_type in types.types.iter() {
          if let Some(_) = sprite_types.insert(sprite_type.id.clone(), sprite_type.clone()) {
            panic!("Conflicting type definitions for id {}", sprite_type.id);
          }
        }
      },
      Err(err) => {
        panic!("Failed to parse sprite types file: {}", err);
      }
    }
  } else {
    panic!("Unable to load sprite types file!");
  }
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource::<SpriteFileVersion>(SpriteFileVersion(1))
      .init_resource::<HashMap<String, SpriteType>>()
      .add_startup_system(load_sprite_types.system());
  }
}
