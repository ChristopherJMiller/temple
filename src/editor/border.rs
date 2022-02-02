use bevy::prelude::*;
use kurinji::Kurinji;

use crate::input::EDIT_TOGGLE_BORDER;
use crate::level::config::SPRITE_SIZE;
use crate::level::load::{LevelLoadComplete, PreparedLevel};
use crate::level::util::get_texture_path;

#[derive(Default)]
pub struct EnableBorder {
  pub toggle_pressed: bool,
  pub enabled: bool,
  pub border_built: bool,
}

pub fn handle_input(mut enable_border: ResMut<EnableBorder>, input: Res<Kurinji>) {
  if input.is_action_active(EDIT_TOGGLE_BORDER) {
    if !enable_border.toggle_pressed {
      enable_border.enabled = !enable_border.enabled;
    }
    enable_border.toggle_pressed = true;
  } else {
    enable_border.toggle_pressed = false;
  }
}

#[derive(Component)]
pub struct SpriteBorder;

pub fn build_border(
  mut commands: Commands,
  mut enable_border: ResMut<EnableBorder>,
  asset_server: Res<AssetServer>,

  loaded_level: Query<&PreparedLevel, With<LevelLoadComplete>>,
) {
  if enable_border.enabled && !enable_border.border_built {
    if let Ok(p_level) = loaded_level.get_single() {
      for sprite in p_level.0.sprites.iter() {
        commands
          .spawn_bundle(SpriteBundle {
            texture: asset_server.load(get_texture_path(&"tileborder.png".to_string())),
            transform: Transform::from_translation(Vec3::new(
              sprite.pos.x as f32 * SPRITE_SIZE as f32,
              sprite.pos.y as f32 * SPRITE_SIZE as f32,
              1.0,
            )),
            ..Default::default()
          })
          .insert(SpriteBorder);
      }

      enable_border.border_built = true;
    }
  }
}

pub fn delete_border(
  mut commands: Commands,
  mut enable_border: ResMut<EnableBorder>,
  border_entities: Query<Entity, With<SpriteBorder>>,
) {
  if !enable_border.enabled && enable_border.border_built {
    border_entities.for_each(|e| {
      commands.entity(e).despawn();
    });
    enable_border.border_built = false;
  }
}

/// [Plugin] for manging a border that can be placed on sprites.
pub struct EditorBorderPlugin;

impl Plugin for EditorBorderPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<EnableBorder>()
      .add_system(handle_input)
      .add_system(build_border)
      .add_system(delete_border);
  }
}
