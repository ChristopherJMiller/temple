use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::vec::Vec;

use bevy::prelude::*;
use png::{BitDepth, ColorType, Decoder};
use serde::Deserialize;

use crate::sprite::{SpritePluginSteps, TempleSprite};

// Level System Loading

pub struct LoadLevel(pub u32);
pub struct LevelLoadComplete;
pub struct UnloadLevel;

pub struct LevelLoadedSprite;

pub const SPRITE_SIZE: u32 = 16;

fn load_level(
  mut commands: Commands,
  query: Query<(Entity, &LoadLevel), Without<LevelLoadComplete>>,
  sprites: Res<HashMap<u32, TempleSprite>>,
  levels: Res<HashMap<u32, Level>>,
) {
  query.for_each(|(e, load_level)| {
    let level_id = load_level.0;

    let level = levels
      .get(&level_id)
      .expect(format!("Attempted to load invalid level id {}", level_id).as_str());

    for sprite in level.sprites.iter() {
      let sprite_data: &TempleSprite = sprites
        .get(&sprite.id)
        .expect(format!("Attempted to load invalid sprite id {}", sprite.id).as_str());

      let transform =
        Transform::from_translation(Vec3::new(sprite.pos.x as f32, sprite.pos.y as f32, 0.0) * SPRITE_SIZE as f32);

      commands
        .spawn_bundle(SpriteBundle {
          material: sprite_data.texture.clone(),
          transform,
          ..Default::default()
        })
        .insert(LevelLoadedSprite);
    }

    info!(target: "load_level", "Loaded Level {}", level_id);
    commands.entity(e).insert(LevelLoadComplete);
  });
}

fn unload_level(
  mut commands: Commands,
  query: Query<Entity, (With<LevelLoadComplete>, With<UnloadLevel>)>,
  level_sprites_query: Query<Entity, With<LevelLoadedSprite>>,
) {
  query.for_each(|e| {
    info!(target: "unload_level", "Unloading level...");
    level_sprites_query.for_each(|sprite| {
      commands.entity(sprite).despawn();
    });

    commands.entity(e).despawn();
  })
}

// Level File Loading

struct LevelFileVersion(u32);

#[derive(Deserialize)]
struct LevelFile {
  version: u32,
  levels: Vec<LevelDefinition>,
}

#[derive(Deserialize)]
struct LevelDefinition {
  id: u32,
  sprite_map: String,
}

pub struct LevelSprite {
  pos: UVec2,
  id: u32,
}

pub struct Level {
  id: u32,
  sprites: Vec<LevelSprite>,
}

fn load_level_files(
  version: Res<LevelFileVersion>,
  sprites: Res<HashMap<u32, TempleSprite>>,
  mut levels: ResMut<HashMap<u32, Level>>,
) {
  let version_num = version.0;

  if let Ok(file) = fs::read_to_string("assets/levels.toml") {
    match toml::from_str::<LevelFile>(file.as_str()) {
      Ok(level_list) => {
        if level_list.version != version_num {
          panic!(
            "Incorrect file version, should be {} but found {}",
            version_num, level_list.version
          );
        }

        for level in level_list.levels.iter() {
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

              for i in (0..info.buffer_size()).step_by(4) {
                // Sprites should not use transparency
                if buf[i + 3] != 255 {
                  continue;
                }

                let x = (i / 4) as u32 % info.width;
                let y = (i / 4) as u32 / info.width;

                let r: u32 = buf[i] as u32;
                let g: u32 = buf[i + 1] as u32;
                let b: u32 = buf[i + 2] as u32;

                let sprite_id: u32 = (r << 8) | (g << 4) | (b << 0);

                if !sprites.contains_key(&sprite_id) {
                  panic!("Attempted to register level with invalid sprite id {}", sprite_id);
                }

                level_sprites.push(LevelSprite {
                  id: sprite_id,
                  pos: UVec2::new(x, y),
                });
              }

              let level_obj = Level {
                id: level.id,
                sprites: level_sprites,
              };

              if let Some(_) = levels.insert(level.id, level_obj) {
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

// Level Plugin

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource::<LevelFileVersion>(LevelFileVersion(1))
      .init_resource::<HashMap<u32, Level>>()
      .add_startup_system(load_level_files.system().after(SpritePluginSteps::LoadSprites))
      .add_system(load_level.system())
      .add_system(unload_level.system());
  }
}
