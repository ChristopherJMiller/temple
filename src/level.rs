use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::vec::Vec;

use bevy::prelude::*;
use png::{BitDepth, ColorType, Decoder};
use serde::Deserialize;

use crate::game::attributes::*;
use crate::sprite::{SpritePluginSteps, TempleSprite, SpriteMap};
use crate::util::files::LEVEL_FILE_PATH;

pub type LevelId = u32;
pub type LevelMap = HashMap<LevelId, Level>;

// Level System Loading

pub struct LoadLevel(pub LevelId);
pub struct LevelLoadComplete;
pub struct UnloadLevel;

pub struct LevelLoadedSprite;

pub const SPRITE_SIZE: u32 = 16;

fn add_component_by_attribute_name(commands: &mut Commands, entity: Entity, name: String) {
  match name.as_str() {
    "solid" => {
      commands.entity(entity).insert(Solid);
    },
    "player" => {
      commands.entity(entity).insert(Player);
    },
    _ => panic!("Attempted to load invalid attribute with name {}", name),
  }
}

fn load_level(
  mut commands: Commands,
  query: Query<(Entity, &LoadLevel), Without<LevelLoadComplete>>,
  sprites: Res<SpriteMap>,
  levels: Res<LevelMap>,
) {
  query.for_each(|(e, load_level)| {
    let level_id = load_level.0;

    let level = levels
      .get(&level_id)
      .unwrap_or_else(|| panic!("Attempted to load invalid level id {}", level_id));

    for sprite in level.sprites.iter() {
      let sprite_data: &TempleSprite = sprites
        .get(&sprite.id)
        .unwrap_or_else(|| panic!("Attempted to load invalid sprite id {}", sprite.id));

      let transform =
        Transform::from_translation(Vec3::new(sprite.pos.x as f32, sprite.pos.y as f32, 0.0) * SPRITE_SIZE as f32);

      let entity = commands
        .spawn_bundle(SpriteBundle {
          material: sprite_data.texture.clone(),
          transform,
          ..Default::default()
        })
        .insert(LevelLoadedSprite)
        .id();

      for attribute in sprite_data.attributes.iter() {
        add_component_by_attribute_name(&mut commands, entity, attribute.clone());
      }
    }

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 1.0 / 4.0;
  
    commands.spawn_bundle(camera).insert(LevelLoadedSprite);

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
pub struct LevelFile {
  version: u32,
  levels: Vec<LevelDefinition>,
}

#[derive(Deserialize)]
struct LevelDefinition {
  id: LevelId,
  sprite_map: String,
}

pub struct LevelSprite {
  pos: UVec2,
  id: u32,
}

pub struct Level {
  #[allow(dead_code)]
  id: LevelId,
  sprites: Vec<LevelSprite>,
}

fn load_level_files(
  version: Res<LevelFileVersion>,
  sprites: Res<SpriteMap>,
  mut levels: ResMut<LevelMap>,
) {
  let version_num = version.0;

  if let Ok(file) = fs::read_to_string(LEVEL_FILE_PATH) {
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

                let level_x = (i / 4) as u32 % info.width;
                let level_y = info.height - ((i / 4) as u32 / info.width);

                let tile_r: u32 = buf[i] as u32;
                let tile_g: u32 = buf[i + 1] as u32;
                let tile_b: u32 = buf[i + 2] as u32;

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

// Level Plugin

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource::<LevelFileVersion>(LevelFileVersion(1))
      .init_resource::<LevelMap>()
      .add_startup_system(load_level_files.system().after(SpritePluginSteps::LoadSprites))
      .add_system(load_level.system())
      .add_system(unload_level.system());
  }
}
