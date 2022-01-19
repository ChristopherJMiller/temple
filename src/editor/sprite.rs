use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use kurinji::Kurinji;

use super::camera::EditorCamera;
use super::ui::{EditorState, EDITOR_ERASER_NAME};
use crate::input::{RETURN, SELECT};
use crate::level::config::{HandledSprite, LevelSpriteEntry, SPRITE_SIZE};
use crate::level::load::{LevelLoadComplete, LevelLoadedSprite, PreparedLevel};
use crate::level::util::load_sprite_texture;

#[derive(Default)]
pub struct SelectedSprite(pub Option<LevelSpriteEntry>);
pub struct SelectedSpriteEntity(pub String, pub LevelSpriteEntry);

pub fn create_selected_sprite_cursor(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  query: Query<(Entity, &SelectedSpriteEntity)>,
  selected_sprite: Res<SelectedSprite>,
) {
  if let Some(sprite) = &selected_sprite.0 {
    if let Ok((ent, tag)) = query.single() {
      if tag.0.ne(&sprite.name) {
        info!(target: "create_selected_sprite_cursor", "Deleting cursor");
        commands.entity(ent).despawn();
      }
    } else {
      info!(target: "create_selected_sprite_cursor", "Creating cursor");
      // Create cursor if does not exist
      commands
        .spawn_bundle(SpriteBundle {
          material: load_sprite_texture(&asset_server, &mut materials, &sprite.texture),
          transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
          ..Default::default()
        })
        .insert(SelectedSpriteEntity(sprite.name.clone(), sprite.clone()));
    }
  } else {
    if let Ok((ent, _)) = query.single() {
      info!(target: "create_selected_sprite_cursor", "Deleting cursor");
      commands.entity(ent).despawn();
    }
  }
}

pub fn handle_selected_sprite(
  win: Res<Windows>,
  selected_sprite: Res<SelectedSprite>,
  proj: Query<&OrthographicProjection, With<EditorCamera>>,
  mut query_set: QuerySet<(
    Query<&Transform, With<EditorCamera>>,
    Query<&mut Transform, With<SelectedSpriteEntity>>,
  )>,
) {
  if selected_sprite.0.is_some() {
    let camera = query_set.q0().single().expect("failed to find editor camera").clone();
    if let Ok(mut cursor) = query_set.q1_mut().single_mut() {
      let window = win.get_primary().unwrap();
      let size = Vec2::new(window.width(), window.height());
      if let Some(pos) = window.cursor_position() {
        let centered_pos = pos - size / 2.0;
        let cursor_singlescreen_pos = centered_pos * proj.single().unwrap().scale;
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
  sprite_on_cursor: Query<(&SelectedSpriteEntity, &Handle<ColorMaterial>, &Transform)>,
  mut loaded_level: Query<&mut PreparedLevel, With<LevelLoadComplete>>,
  loaded_sprites: Query<(Entity, &Transform), With<LevelLoadedSprite>>,
  input: Res<Kurinji>,
  mut editor_state: ResMut<EditorState>,
) {
  // Cursor is active
  if let Ok((sprite, material, transform)) = sprite_on_cursor.single() {
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
          let mut level = loaded_level.single_mut().unwrap();
          let handled_sprite: HandledSprite = (sprite.1.clone(), tile_pos, material.clone()).into();
          level.0.sprites.push(handled_sprite);
          commands.spawn_bundle(SpriteBundle {
            material: material.clone(),
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
        let sprites = loaded_level.single_mut().unwrap().0.sprites.clone();
        for (i, sprite) in sprites.iter().enumerate() {
          if sprite.pos.eq(&tile_pos) {
            loaded_level.single_mut().unwrap().0.sprites.swap_remove(i);
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
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<SelectedSprite>()
      .add_system(create_selected_sprite_cursor.system())
      .add_system(handle_selected_sprite.system())
      .add_system(handle_placing_sprite.system())
      .add_system(handle_deselect.system());
  }
}
