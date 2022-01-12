use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

use super::camera::EditorCamera;
use crate::level::config::{LevelSpriteEntry, SPRITE_SIZE};
use crate::level::util::load_sprite_texture;

#[derive(Default)]
pub struct SelectedSprite(pub Option<LevelSpriteEntry>);
pub struct SelectedSpriteEntity(pub String);

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
        .insert(SelectedSpriteEntity(sprite.name.clone()));
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

/// [Plugin] for manging level sprites within the editor
pub struct EditorSpritePlugin;

impl Plugin for EditorSpritePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<SelectedSprite>()
      .add_system(create_selected_sprite_cursor.system())
      .add_system(handle_selected_sprite.system());
  }
}
