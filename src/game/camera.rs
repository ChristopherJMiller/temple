use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::game::attributes::Player;

pub struct CameraTarget;

fn target_camera(
  mut camera: Query<&mut Transform, (With<Camera>, Without<CameraPlugin>, Without<Player>)>,
  targets: Query<&Transform, (Without<Camera>, With<CameraTarget>, Without<Player>)>,
  player: Query<&Transform, (Without<Camera>, Without<CameraTarget>, With<Player>)>,
) {
  if let Ok(mut camera_trans) = camera.single_mut() {
    if let Ok(target) = targets.single() {
      camera_trans.translation = target.translation;
    } else {
      if let Ok(player_trans) = player.single() {
        camera_trans.translation = player_trans.translation;
      }
    }
  }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(target_camera.system());
  }
}
