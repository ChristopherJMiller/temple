use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use kurinji::Kurinji;

use super::camera::EditorCamera;
use super::ui::{EditorState, EDITOR_ERASER_NAME};
use crate::input::{RETURN, SELECT};
use crate::level::config::{HandledSprite, LevelSpriteEntry, SPRITE_SIZE};
use crate::level::load::{LevelLoadComplete, LevelLoadedSprite, PreparedLevel};
use crate::level::util::get_texture_path;

#[derive(Default)]
pub struct SelectedSprite(pub Option<LevelSpriteEntry>);
#[derive(Component)]
pub struct SelectedSpriteEntity(pub String, pub LevelSpriteEntry);

pub fn create_selected_sprite_cursor(
  mut commands: Commands,
  asset_server: Res<AssetServer>,

  query: Query<(Entity, &SelectedSpriteEntity)>,
  selected_sprite: Res<SelectedSprite>,
) {
  if let Some(sprite) = &selected_sprite.0 {
    if let Ok((ent, tag)) = query.get_single() {
      if tag.0.ne(&sprite.name) {
        info!(target: "create_selected_sprite_cursor", "Deleting cursor");
        commands.entity(ent).despawn();
      }
    } else {
      info!(target: "create_selected_sprite_cursor", "Creating cursor");
      // Create cursor if does not exist
      commands
        .spawn_bundle(SpriteBundle {
          texture: asset_server.load(get_texture_path(&sprite.texture)),
          transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
          ..Default::default()
        })
        .insert(SelectedSpriteEntity(sprite.name.clone(), sprite.clone()));
    }
  } else {
    if let Ok((ent, _)) = query.get_single() {
      info!(target: "create_selected_sprite_cursor", "Deleting cursor");
      commands.entity(ent).despawn();
    }
  }
}

pub fn handle_selected_sprite(
  win: Res<Windows>,
  selected_sprite: Res<SelectedSprite>,
  proj: Query<&OrthographicProjection, With<EditorCamera>>,
  camera_query: Query<&Transform, With<EditorCamera>>,
  mut trans_query: Query<&mut Transform, With<SelectedSpriteEntity>>,
) {
  if selected_sprite.0.is_some() {
    let camera = camera_query.get_single().expect("failed to find editor camera").clone();
    if let Ok(mut cursor) = trans_query.get_single_mut() {
      let window = win.get_primary().unwrap();
      let size = Vec2::new(window.width(), window.height());
      if let Some(pos) = window.cursor_position() {
        let centered_pos = pos - size / 2.0;
        let cursor_singlescreen_pos = centered_pos * proj.get_single().unwrap().scale;
        let cursor_pos = camera.compute_matrix() * cursor_singlescreen_pos.extend(0.0).extend(1.0);
        let grid_pos = (cursor_pos / SPRITE_SIZE as f32).round() * SPRITE_SIZE as f32;
        let sprite_transform = Vec3::new(grid_pos.x, grid_pos.y, 0.0);
        cursor.translation = sprite_transform;
      }
    }
  }
}

pub fn handle_placing_sprite(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  sprite_on_cursor: Query<(&SelectedSpriteEntity, &Transform)>,
  mut loaded_level: Query<&mut PreparedLevel, With<LevelLoadComplete>>,
  loaded_sprites: Query<(Entity, &Transform), With<LevelLoadedSprite>>,
  input: Res<Kurinji>,
  mut editor_state: ResMut<EditorState>,
) {
  // Cursor is active
  if let Ok((sprite, transform)) = sprite_on_cursor.get_single() {
    // "Left Click" is activated
    if input.is_action_active(SELECT) {
      let pos = IVec2::new(transform.translation.x as i32, transform.translation.y as i32);
      let tile_pos = pos / SPRITE_SIZE as i32;
      // Nothing placed in that position
      if !editor_state.placed_sprites.contains_key(&tile_pos) {
        // Make sure it's not the eraser
        if sprite.0.ne(EDITOR_ERASER_NAME) {
          // Insert
          info!(target: "handle_placing_sprite", "Inserted");
          editor_state.placed_sprites.insert(tile_pos, sprite.0.clone());
          let mut level = loaded_level.get_single_mut().unwrap();
          let handled_sprite: HandledSprite = (sprite.1.clone(), tile_pos).into();
          level.0.sprites.push(handled_sprite.clone());
          commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load(handled_sprite.texture.clone().as_str()),
            transform: transform.clone(),
            ..Default::default()
          });
        }
      // If it does exist, and using the eraser, delete the sprite
      } else if sprite.0.eq(EDITOR_ERASER_NAME) {
        // Remove from table
        info!(target: "handle_placing_sprite", "Deleted");
        editor_state.placed_sprites.remove(&tile_pos);

        // Remove from level entries
        let sprites = loaded_level.get_single_mut().unwrap().0.sprites.clone();
        for (i, sprite) in sprites.iter().enumerate() {
          if sprite.pos.eq(&tile_pos) {
            loaded_level.get_single_mut().unwrap().0.sprites.swap_remove(i);
            break;
          }
        }

        // Remove from bevy world
        for (entity, trans) in loaded_sprites.iter() {
          if trans.translation == Vec3::new(pos.x as f32, pos.y as f32, 0.0) {
            commands.entity(entity).despawn();
          }
        }
      }
    }
  }
}

pub fn handle_deselect(input: Res<Kurinji>, mut selected_sprite: ResMut<SelectedSprite>) {
  if input.is_action_active(RETURN) {
    if selected_sprite.0.is_some() {
      selected_sprite.0 = None;
    }
  }
}

/// [Plugin] for manging level sprites within the editor
pub struct EditorSpritePlugin;

impl Plugin for EditorSpritePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<SelectedSprite>()
      .add_system(create_selected_sprite_cursor)
      .add_system(handle_selected_sprite)
      .add_system(handle_placing_sprite)
      .add_system(handle_deselect);
  }
}
