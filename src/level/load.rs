//! Handles level loading for gameplay.
//!
//! # Usage
//! Levels are instructed to load using tag [LoadLevel].
//! [LevelLoadComplete] can be tracked for completion of load.
//! All sprites that are loaded via [LoadLevel] are tagged with
//! [LevelLoadedSprite]. Instruction [UnloadLevel] can be added to the original
//! [LoadLevel] entity to instruct an unload.

use bevy::prelude::*;
use bevy_rapier2d::physics::TimestepMode;
use bevy_rapier2d::prelude::*;

use super::{LevelId, LevelMap};
use crate::game::attributes::*;
use crate::game::camera::MainCamera;
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

/// Startup system to configure rapier physics for sprites
pub fn configure_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
  rapier_config.scale = 1.0;
  rapier_config.timestep_mode = TimestepMode::FixedTimestep;
}

/// System that loads sprites in a given level.
/// Can be tracked with [LevelLoadComplete]
pub fn load_level(
  mut commands: Commands,
  query: Query<(Entity, &LoadLevel), Without<LevelLoadComplete>>,
  sprites: Res<SpriteMap>,
  levels: Res<LevelMap>,
) {
  query.for_each(|(e, load_level)| {
    let level_id = load_level.0;

    // Get level by id
    let level = levels
      .get(&level_id)
      .unwrap_or_else(|| panic!("Attempted to load invalid level id {}", level_id));

    // Get all sprites in level
    for sprite in level.sprites.iter() {
      let sprite_data: &GameSprite = sprites
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
        let position = Vec2::new(transform.translation.x, transform.translation.y);
        build_attribute(attribute.clone(), &mut commands, entity, position);
      }
    }

    let mut camera = OrthographicCameraBundle::new_2d();
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
