use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};
use kurinji::Kurinji;

use super::ui::EditorState;
use crate::input::{DOWN, EDIT_ZOOM_IN, EDIT_ZOOM_OUT, LEFT, RIGHT, UP};

#[derive(Component)]
pub struct EditorCamera;

pub fn handle_camera_input(
  input: Res<Kurinji>,
  time: Res<Time>,
  move_speed: Res<CameraMoveSpeed>,
  editor_state: Res<EditorState>,
  mut camera: Query<&mut Transform, With<EditorCamera>>,
) {
  if editor_state.ui_open() {
    return;
  }

  if let Ok(mut trans) = camera.get_single_mut() {
    if input.is_action_active(UP) {
      trans.translation.y += time.delta_seconds() * move_speed.0;
    }

    if input.is_action_active(RIGHT) {
      trans.translation.x += time.delta_seconds() * move_speed.0;
    }

    if input.is_action_active(DOWN) {
      trans.translation.y -= time.delta_seconds() * move_speed.0;
    }

    if input.is_action_active(LEFT) {
      trans.translation.x -= time.delta_seconds() * move_speed.0;
    }
  }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
  a + (b - a) * t
}

fn get_camera_speed(t: f32) -> f32 {
  lerp(16.0, 1024.0, t)
}

pub fn handle_camera_zooming(
  input: Res<Kurinji>,
  time: Res<Time>,
  mut speed: ResMut<CameraMoveSpeed>,
  editor_state: Res<EditorState>,
  mut camera: Query<(&mut Camera, &mut OrthographicProjection), With<EditorCamera>>,
) {
  if editor_state.ui_open() {
    return;
  }

  if let Ok((mut camera, mut proj)) = camera.get_single_mut() {
    if input.is_action_active(EDIT_ZOOM_IN) {
      proj.scale = (proj.scale + time.delta_seconds()).min(2.0);
      camera.projection_matrix = Mat4::orthographic_rh(
        proj.left * proj.scale,
        proj.right * proj.scale,
        proj.bottom * proj.scale,
        proj.top * proj.scale,
        proj.near,
        proj.far,
      );
      speed.0 = get_camera_speed(proj.scale);
    } else if input.is_action_active(EDIT_ZOOM_OUT) {
      proj.scale = (proj.scale - time.delta_seconds()).max(0.05);
      camera.projection_matrix = Mat4::orthographic_rh(
        proj.left * proj.scale,
        proj.right * proj.scale,
        proj.bottom * proj.scale,
        proj.top * proj.scale,
        proj.near,
        proj.far,
      );
      speed.0 = get_camera_speed(proj.scale);
    }
  }
}

pub struct CameraMoveSpeed(pub f32);

/// [Plugin] for camera systems.
pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(CameraMoveSpeed(get_camera_speed(1.0 / 3.0)))
      .add_system(handle_camera_input)
      .add_system(handle_camera_zooming);
  }
}
