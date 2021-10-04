//! Camera related systems.
//!
//! The camera system is designed with cutscene-like target focusing in mind.
//! Targeting is defined as follows:
//! - If a [CameraTarget] is defined, it takes camera focus.
//! - Otherwise, the [Player] is focused.

use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::game::attributes::Player;

/// Tag for a non-player camera focus.
pub struct CameraTarget;

/// Tag for declaring the main camera (as opposed to UI cameras, etc.)
pub struct MainCamera;

/// Component filtering for the camera.
type CameraOnly = (With<Camera>, With<MainCamera>, Without<CameraPlugin>, Without<Player>);

/// Component filtering for any [CameraTarget]'s.
type CameraTargetOnly = (Without<Camera>, With<CameraTarget>, Without<Player>);

/// Component filtering for the player.
type PlayerOnly = (Without<Camera>, Without<CameraTarget>, With<Player>);


/// System to determine camera targeting.
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

/// [Plugin] for camera systems.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(target_camera.system());
  }
}
