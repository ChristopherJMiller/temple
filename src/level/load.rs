//! Handles level loading for gameplay.
//!
//! # Usage
//! Levels are instructed to load using tag [LoadLevel].
//! [LevelLoadComplete] can be tracked for completion of load.
//! All sprites that are loaded via [LoadLevel] are tagged with
//! [LevelLoadedSprite]. Instruction [UnloadLevel] can be added to the original
//! [LoadLevel] entity to instruct an unload.

use bevy::asset::LoadState;
use bevy::ecs::system::QuerySingleError;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioSource};
use bevy_rapier2d::prelude::RigidBodyPositionComponent;

use super::config::{Level, LevelManifest, LevelMap};
use super::util::{get_manifest_by_id, get_map_by_id, levels_have_same_music, prepare_level_from_manifests};
use super::LevelId;
use crate::editor::camera::EditorCamera;
use crate::game::attributes::*;
use crate::game::camera::MainCamera;
use crate::game::sfx::AudioChannels;
use crate::input::CursorCommands;
use crate::level::config::SPRITE_SIZE;
use crate::level::util::get_texture_path;
use crate::state::game_state::{ActiveSave, GameMode, TempleState};
use crate::ui::overlay::{OverlayCommand, OverlayCommands};
use crate::util::files::{from_game_root, MUSIC_DIR_PATH};

/// Instruction to load a new level
#[derive(Component)]
pub struct LoadLevel(pub LevelId);

/// Loaded level manifests, as part of the load level process
#[derive(Component)]
pub struct PreparedLevel(pub Level);

/// Tag that LoadLevel has completed. Added to same entity as [LoadLevel]
#[derive(Component)]
pub struct LevelLoadComplete;

/// Instruction to unload a level. Must be added to the same entity as
/// [LoadLevel]
#[derive(Component)]
pub struct UnloadLevel;

/// Tag that entity was loaded by level, and will be removed when [UnloadLevel]
/// instruction is given
#[derive(Component)]
pub struct LevelLoadedSprite;

/// Instruction to unload current level and transition to another.
#[derive(Component, Debug)]
pub struct TransitionLevel(pub LevelId);

/// Instruction to not [LoadLevel] until previous level is unloaded.
#[derive(Component)]
pub struct WaitUntilUnloaded;

/// Instruction used by transition level to prevent the same music from being
/// replayed.
#[derive(Component)]
pub struct KeepMusic;

/// Instruction to skip to next checkpoint
#[derive(Component)]
pub struct NextCheckpoint;

pub fn wait_until_unloaded(
  mut commands: Commands,
  loaded_level: Query<(Entity, &LoadLevel), (With<LevelLoadComplete>, Without<WaitUntilUnloaded>)>,
  new_level: Query<Entity, (With<LoadLevel>, With<WaitUntilUnloaded>)>,
) {
  if let Ok(ent) = new_level.get_single() {
    match loaded_level.get_single() {
      Ok(_) => {},
      Err(err) => match err {
        QuerySingleError::NoEntities(_) => {
          commands.entity(ent).remove::<WaitUntilUnloaded>();
        },
        QuerySingleError::MultipleEntities(_) => {
          warn!(target: "wait_until_unloaded", "Multiple fully loaded levels detected. Things may get weird...");
        },
      },
    }
  }
}

/// System that prepares the level from files, to be loaded by [load_level]
pub fn prepare_level(
  mut commands: Commands,
  active_save: Res<ActiveSave>,

  temple_state: Res<TempleState>,
  mut query: Query<(Entity, &mut LoadLevel), (Without<PreparedLevel>, Without<LevelLoadComplete>)>,
) {
  query.for_each_mut(|(e, mut load_level)| {
    info!(target: "prepare_level", "Loading level {}", load_level.0);
    let in_edit_mode = temple_state.in_edit_mode();
    // Load checkpoint in play mode if avaliable
    if !in_edit_mode {
      if let Some(level_state) = active_save.get_level_state(load_level.0) {
        if let Some((new_id, _, _)) = level_state.checkpoint() {
          if load_level.0 != *new_id {
            info!(target: "prepare_level", "Checkpoint save detected, changing load_level to {}", *new_id);
            load_level.0 = *new_id;
          }
        }
      }
    }

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

    let level = prepare_level_from_manifests(manifest, map);
    commands.entity(e).insert(PreparedLevel(level));
  });
}

/// System that loads sprites in a given level.
/// Can be tracked with [LevelLoadComplete]
pub fn load_level(
  mut commands: Commands,
  query: Query<(Entity, &LoadLevel, &PreparedLevel), (Without<LevelLoadComplete>, Without<WaitUntilUnloaded>)>,
  keep_music: Query<&KeepMusic>,
  asset_server: Res<AssetServer>,
  temple_state: Res<TempleState>,
  audio: Res<Audio>,
  channels: Res<AudioChannels>,
  mut overlay_commands: ResMut<OverlayCommands>,
  mut cursor_commands: ResMut<CursorCommands>,
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
        // Play new music if load is lacking [KeepMusic] instruction
        if keep_music.get_component::<KeepMusic>(e).is_err() {
          // Workaround due to Bevy reloading previous asset issue,
          // Gets a valid handle, and since was previously loaded does not need to load
          // from disk.
          audio.play_looped_in_channel(
            asset_server.load(music_path.into_os_string().to_str().unwrap()),
            &channels.music.0,
          );
        }
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
          texture: asset_server.load(get_texture_path(&sprite.texture)),
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
        build_attribute(attribute.clone(), &mut commands, entity, level_id, position);
      }
    }

    if !in_edit_mode {
      let mut camera = OrthographicCameraBundle::new_2d();
      camera.transform.translation = player_trans;
      camera.transform.translation.z = 16.0;
      camera.orthographic_projection.scale = 1.0 / 3.0;

      commands
        .spawn_bundle(camera)
        .insert(LevelLoadedSprite)
        .insert(MainCamera);
    } else {
      let mut camera = OrthographicCameraBundle::new_2d();
      camera.transform.translation = player_trans;
      camera.transform.translation.z = 16.0;
      camera.orthographic_projection.scale = 1.0 / 3.0;

      commands
        .spawn_bundle(camera)
        .insert(LevelLoadedSprite)
        .insert(EditorCamera);
    }

    info!(target: "load_level", "Loaded Level {}", level_id);
    if !temple_state.in_edit_mode() {
      cursor_commands.lock_cursor();
      overlay_commands.command(OverlayCommand::FadeOut(1.0));
    }
    commands.entity(e).insert(LevelLoadComplete);
  });
}

/// Tag to track a level having save files applied to it.
#[derive(Component)]
pub struct LevelSaveApplied;

/// Applies the checkpoint location if an active save warrants it.
pub fn apply_save_on_load(
  mut commands: Commands,
  mut player: Query<(&mut RigidBodyPositionComponent, &mut Player)>,
  level: Query<(Entity, &LoadLevel), (With<LevelLoadComplete>, Without<LevelSaveApplied>)>,
  temple_state: Res<TempleState>,
  active_save: Res<ActiveSave>,
) {
  if let Ok((ent, load_level)) = level.get_single() {
    if let GameMode::InLevel(id) = temple_state.game_mode {
      if let Some(level_state) = active_save.get_level_state(id) {
        if let Some((id, x, y)) = level_state.checkpoint() {
          if let Ok((mut trans, mut player)) = player.get_single_mut() {
            let pos = Vec2::new(*x, *y);
            player.respawn_pos = pos;
            player.respawn_level = *id;
            if load_level.0 == *id {
              trans.position.translation = pos.into();
            }
            info!(target: "apply_save_on_load", "Applied save, respawn is now {} ({}, {})", *id, *x, *y);
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

/// System that consumes [TransitionLevel] instructions
/// Produces
pub fn transition_level(
  mut commands: Commands,
  audio: Res<Audio>,
  channels: Res<AudioChannels>,
  transition_command: Query<(Entity, &TransitionLevel)>,
  loaded_level: Query<(Entity, &LoadLevel), With<LevelLoadComplete>>,
  loading_levels: Query<Entity, (With<LoadLevel>, Without<LevelLoadComplete>)>,
) {
  let mut transitioned = false;
  for (entity, trans) in transition_command.iter() {
    // Run a single transition, prevents multiple transition entities on the same
    // frame.
    if !transitioned {
      transitioned = true;
      // Do nothing if already a load active, prevents double loads.
      if loading_levels.get_single().is_err() {
        if let Ok((ent, old_level)) = loaded_level.get_single() {
          commands.entity(ent).insert(UnloadLevel);
          let mut level_ent = commands.spawn();
          level_ent.insert(LoadLevel(trans.0)).insert(WaitUntilUnloaded);
          if levels_have_same_music(old_level.0, trans.0) {
            level_ent.insert(KeepMusic);
          } else {
            audio.stop_channel(&channels.music.0);
          }
        } else {
          warn!(target: "transition_level", "Unable to get currently loaded level! None or More than 1 Loaded?");
        }
      }
    }

    commands.entity(entity).despawn();
  }
}

/// Attempts to move player to the next checkpoint, if one exists in this level.
pub fn next_checkpoint(
  mut commands: Commands,
  inst: Query<Entity, With<NextCheckpoint>>,
  mut player: Query<(&Player, &mut RigidBodyPositionComponent)>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  checkpoints: Query<&Checkpoint>,
) {
  inst.for_each(|ent| {
    if let Ok(level) = loaded_level.get_single() {
      if let Ok((player, mut pos)) = player.get_single_mut() {
        if player.respawn_level == level.0 {
          // See if player is at checkpoint
          let current_checkpoint = checkpoints.iter().find(|x| x.1 == player.respawn_pos);
          // If so, set to next checkpoint
          if let Some(checkpoint) = current_checkpoint {
            let next_checkpoint = checkpoints.iter().find(|x| x.0 == checkpoint.0 + 1);
            if let Some(new_checkpoint) = next_checkpoint {
              pos.0.position = new_checkpoint.1.into();
            }
          } else {
            // Otherwise, they probably don't have a checkpoint, so give them id 0
            let next_checkpoint = checkpoints.iter().find(|x| x.0 == 0);
            if let Some(new_checkpoint) = next_checkpoint {
              pos.0.position = new_checkpoint.1.into();
            }
          }
        }
      }
    }

    commands.entity(ent).despawn();
  });
}
