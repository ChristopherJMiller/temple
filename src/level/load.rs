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
use bevy_rapier2d::prelude::RigidBodyPosition;

use super::config::{Level, LevelManifest, LevelMap};
use super::util::{get_manifest_by_id, get_map_by_id, prepare_level_from_manifests};
use super::LevelId;
use crate::editor::camera::EditorCamera;
use crate::game::attributes::*;
use crate::game::camera::MainCamera;
use crate::game::sfx::AudioChannels;
use crate::level::config::SPRITE_SIZE;
use crate::state::game_state::{ActiveSave, GameSaveState, LevelClearState, TempleState};
use crate::util::files::{from_game_root, MUSIC_DIR_PATH};

/// Instruction to load a new level
pub struct LoadLevel(pub LevelId);

/// Loaded level manifests, as part of the load level process
pub struct PreparedLevel(pub Level);

/// Tag that LoadLevel has completed. Added to same entity as [LoadLevel]
pub struct LevelLoadComplete;

/// Instruction to unload a level. Must be added to the same entity as
/// [LoadLevel]
pub struct UnloadLevel;

/// Tag that entity was loaded by level, and will be removed when [UnloadLevel]
/// instruction is given
pub struct LevelLoadedSprite;

/// System that prepares the level from files, to be loaded by [load_level]
pub fn prepare_level(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  temple_state: Res<TempleState>,
  query: Query<(Entity, &LoadLevel), (Without<PreparedLevel>, Without<LevelLoadComplete>)>,
) {
  query.for_each(|(e, load_level)| {
    let in_edit_mode = temple_state.in_edit_mode();
    let id = load_level.0;
    // If in edit mode, a lack of manifest is forgiven.
    let manifest = if let Some(manifest) = get_manifest_by_id(id) {
      manifest
    } else if in_edit_mode {
      LevelManifest::default()
    } else {
      panic!("Attempted to load invalid level manifest {}", id)
    };

    // If in edit mode, a lack of level map is forgiven.
    let map = if let Some(map) = get_map_by_id(id) {
      map
    } else if in_edit_mode {
      LevelMap::default()
    } else {
      panic!("Attempted to load invalid level map {}", id)
    };

    let level = prepare_level_from_manifests(&asset_server, manifest, map);
    commands.entity(e).insert(PreparedLevel(level));
  });
}

/// System that loads sprites in a given level.
/// Can be tracked with [LevelLoadComplete]
pub fn load_level(
  mut commands: Commands,
  query: Query<(Entity, &LoadLevel, &PreparedLevel), Without<LevelLoadComplete>>,
  asset_server: Res<AssetServer>,
  temple_state: Res<TempleState>,
  audio: Res<Audio>,
  channels: Res<AudioChannels>,
) {
  query.for_each(|(e, load_level, prepared_level)| {
    let level_id = load_level.0;
    let in_edit_mode = temple_state.in_edit_mode();
    let level = &prepared_level.0;

    let music_path = from_game_root(MUSIC_DIR_PATH).join(level.music.clone());
    let music: Handle<AudioSource> = asset_server.get_handle(music_path.clone().into_os_string().to_str().unwrap());

    // Ensure sprites are loaded
    for sprite in &level.sprites {
      if asset_server.get_load_state(sprite.texture.clone()) == LoadState::Loading {
        // Wait for Load
        return;
      }
    }

    // Load music
    if !in_edit_mode {
      if asset_server.get_load_state(&music) == LoadState::Loaded {
        audio.play_looped_in_channel(music, &channels.music.0);
      } else if asset_server.get_load_state(&music) != LoadState::Loading {
        let _: Handle<AudioSource> = asset_server.load(music_path.into_os_string().to_str().unwrap());
        return;
      } else {
        // Wait for load
        return;
      }
    }

    let mut player_trans = Vec3::ZERO;

    // Get all sprites in level
    for sprite in level.sprites.iter() {
      let unit_pos = Vec3::new(sprite.pos.x as f32, sprite.pos.y as f32, 0.0);
      let sprite_pos = unit_pos * SPRITE_SIZE as f32;

      let entity = commands
        .spawn_bundle(SpriteBundle {
          material: sprite.texture.clone(),
          transform: Transform::from_translation(sprite_pos),
          ..Default::default()
        })
        .insert(LevelLoadedSprite)
        .id();

      for attribute in sprite.attributes.iter() {
        let position = Vec2::new(unit_pos.x, unit_pos.y);
        if attribute == Player::KEY {
          player_trans = unit_pos;
        }
        build_attribute(attribute.clone(), &mut commands, entity, position);
      }
    }

    if !in_edit_mode {
      let mut camera = OrthographicCameraBundle::new_2d();
      camera.transform.translation = player_trans;
      camera.orthographic_projection.scale = 1.0 / 3.0;

      commands
        .spawn_bundle(camera)
        .insert(LevelLoadedSprite)
        .insert(MainCamera);
    } else {
      let mut camera = OrthographicCameraBundle::new_2d();
      camera.transform.translation = player_trans;
      camera.orthographic_projection.scale = 1.0 / 3.0;

      commands
        .spawn_bundle(camera)
        .insert(LevelLoadedSprite)
        .insert(EditorCamera);
    }

    info!(target: "load_level", "Loaded Level {}", level_id);
    commands.entity(e).insert(LevelLoadComplete);
  });
}

/// Tag to track a level having save files applied to it.
pub struct LevelSaveApplied;

/// Applies the checkpoint location if an active save warrants it.
pub fn apply_save_on_load(
  mut commands: Commands,
  mut player: Query<(&mut RigidBodyPosition, &mut Player)>,
  level: Query<(Entity, &LoadLevel), (With<LevelLoadComplete>, Without<LevelSaveApplied>)>,
  active_save: Res<ActiveSave>,
) {
  if let Ok((ent, load)) = level.single() {
    if let Some(save) = &active_save.0 {
      if let Some(level_state) = save.level_clears.get(&GameSaveState::key(load.0)) {
        if let LevelClearState::AtCheckpoint(x, y) = level_state {
          if let Ok((mut trans, mut player)) = player.single_mut() {
            let pos = Vec2::new(*x, *y);
            player.respawn_pos = pos;
            trans.position.translation = pos.into();
          }
        }
      }
    }

    commands.entity(ent).insert(LevelSaveApplied);
  }
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
