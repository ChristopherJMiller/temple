//! Handles level loading for gameplay.
//!
//! # Usage
//! Levels are instructed to load using tag [LoadLevel].
//! [LevelLoadComplete] can be tracked for completion of load.
//! All sprites that are loaded via [LoadLevel] are tagged with
//! [LevelLoadedSprite]. Instruction [UnloadLevel] can be added to the original
//! [LoadLevel] entity to instruct an unload.

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioSource};

use super::{LevelId, LevelMap};
use crate::game::attributes::*;
use crate::game::camera::MainCamera;
use crate::game::sfx::AudioChannels;
use crate::sprite::{GameSprite, SpriteMap, SPRITE_SIZE};

/// Instruction to load a new level
pub struct LoadLevel(pub LevelId);

/// Tag that LoadLevel has completed. Added to same entity as [LoadLevel]
pub struct LevelLoadComplete;

/// Instruction to unload a level. Must be added to the same entity as
/// [LoadLevel]
pub struct UnloadLevel;

/// Tag that entity was loaded by level, and will be removed when [UnloadLevel]
/// instruction is given
pub struct LevelLoadedSprite;

/// System that loads sprites in a given level.
/// Can be tracked with [LevelLoadComplete]
pub fn load_level(
  mut commands: Commands,
  query: Query<(Entity, &LoadLevel), Without<LevelLoadComplete>>,
  sprites: Res<SpriteMap>,
  levels: Res<LevelMap>,
  asset_server: Res<AssetServer>,
  audio: Res<Audio>,
  channels: Res<AudioChannels>,
) {
  query.for_each(|(e, load_level)| {
    let level_id = load_level.0;

    // Get level by id
    let level = levels
      .get(&level_id)
      .unwrap_or_else(|| panic!("Attempted to load invalid level id {}", level_id));

    let music: Handle<AudioSource> = asset_server.get_handle(level.music_file.as_str());

    // Load music
    if asset_server.get_load_state(&music) == LoadState::Loaded {
      audio.play_looped_in_channel(music, &channels.music.0);
    } else if asset_server.get_load_state(&music) != LoadState::Loading {
      let _: Handle<AudioSource> = asset_server.load(level.music_file.as_str());
      return;
    } else {
      // Wait for load
      return;
    }

    let mut player_trans = Vec3::ZERO;

    // Get all sprites in level
    for sprite in level.sprites.iter() {
      let sprite_data: &GameSprite = sprites
        .get(&sprite.id)
        .unwrap_or_else(|| panic!("Attempted to load invalid sprite id {}", sprite.id));

      let unit_pos = Vec3::new(sprite.pos.x as f32, sprite.pos.y as f32, 0.0);
      let sprite_pos = unit_pos * SPRITE_SIZE as f32;

      let entity = commands
        .spawn_bundle(SpriteBundle {
          material: sprite_data.texture.clone(),
          transform: Transform::from_translation(sprite_pos),
          ..Default::default()
        })
        .insert(LevelLoadedSprite)
        .id();

      for attribute in sprite_data.attributes.iter() {
        let position = Vec2::new(unit_pos.x, unit_pos.y);
        if attribute == Player::KEY {
          player_trans = unit_pos;
        }
        build_attribute(attribute.clone(), &mut commands, entity, position);
      }
    }

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform.translation = player_trans;
    camera.orthographic_projection.scale = 1.0 / 3.0;

    commands
      .spawn_bundle(camera)
      .insert(LevelLoadedSprite)
      .insert(MainCamera);

    info!(target: "load_level", "Loaded Level {}", level_id);
    commands.entity(e).insert(LevelLoadComplete);
  });
}

/// System that unloads a currently loaded level using the [UnloadLevel] tag
pub fn unload_level(
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
