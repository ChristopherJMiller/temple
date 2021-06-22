use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::game::attributes::Player;

pub struct CameraTarget;

type CameraOnly = (With<Camera>, Without<CameraPlugin>, Without<Player>);
type CameraTargetOnly = (Without<Camera>, With<CameraTarget>, Without<Player>);
type PlayerOnly = (Without<Camera>, Without<CameraTarget>, With<Player>);

fn target_camera(
  mut camera: Query<&mut Transform, CameraOnly>,
  targets: Query<&Transform, CameraTargetOnly>,
  player: Query<&Transform, PlayerOnly>,
) {
  if let Ok(mut camera_trans) = camera.single_mut() {
    if let Ok(target) = targets.single() {
      camera_trans.translation = target.translation;
    } else if let Ok(player_trans) = player.single() {
      camera_trans.translation = player_trans.translation;
    }
  }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(target_camera.system());
  }
}
