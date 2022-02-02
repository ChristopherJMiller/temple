//! Camera related systems.
//!
//! The camera system is designed with cutscene-like target focusing in mind.
//! Targeting is defined as follows:
//! - If a [CameraTarget] is defined, it takes camera focus.
//! - Otherwise, the [Player] is focused.

use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::game::attributes::Player;
use crate::level::config::SPRITE_SIZE;

/// Tag for a non-player camera focus.
#[derive(Component)]
pub struct CameraTarget;

/// Tag for declaring the main camera (as opposed to UI cameras, etc.)
#[derive(Component)]
pub struct MainCamera;

/// Component filtering for the camera.
type CameraOnly = (With<Camera>, With<MainCamera>, Without<Player>);

/// Component filtering for any [CameraTarget]'s.
type CameraTargetOnly = (Without<Camera>, With<CameraTarget>, Without<Player>);

/// Component filtering for the player.
type PlayerOnly = (Without<Camera>, Without<CameraTarget>, With<Player>);

/// System to determine camera targeting.
fn target_camera(
  time: Res<Time>,
  cam_speed: Res<CameraTrackingSpeed>,
  mut camera: Query<&mut Transform, CameraOnly>,
  targets: Query<&Transform, CameraTargetOnly>,
  player: Query<&Transform, PlayerOnly>,
) {
  if let Ok(mut camera_trans) = camera.get_single_mut() {
    let target = if let Ok(target) = targets.get_single() {
      target.translation.truncate()
    } else if let Ok(player_trans) = player.get_single() {
      player_trans.translation.truncate()
    } else {
      camera_trans.translation.truncate()
    };

    // Snap to target if very far away
    if target.distance_squared(camera_trans.translation.truncate()) > 1000.0 {
      camera_trans.translation = Vec3::new(target.x, target.y, camera_trans.translation.z);
      return;
    }

    let diff_vec = target - camera_trans.translation.truncate();

    let dir = diff_vec.normalize_or_zero();
    let speed = (diff_vec.length() * SPRITE_SIZE as f32).min(cam_speed.0);

    camera_trans.translation += dir.extend(0.0) * speed * time.delta_seconds();
  }
}

/// [Plugin] for camera systems.
pub struct CameraPlugin;

/// [Res] for how fast the camera can move per frame.
pub struct CameraTrackingSpeed(pub f32);

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource::<CameraTrackingSpeed>(CameraTrackingSpeed(SPRITE_SIZE as f32 * 64.0))
      .add_system(target_camera);
  }
}
