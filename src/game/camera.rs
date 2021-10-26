//! Camera related systems.
//!
//! The camera system is designed with cutscene-like target focusing in mind.
//! Targeting is defined as follows:
//! - If a [CameraTarget] is defined, it takes camera focus.
//! - Otherwise, the [Player] is focused.

use bevy::prelude::*;
use bevy::render::camera::Camera;

use crate::game::attributes::Player;
use crate::sprite::SPRITE_SIZE;

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
  time: Res<Time>,
  cam_speed: Res<CameraTrackingSpeed>,
  mut camera: Query<&mut Transform, CameraOnly>,
  targets: Query<&Transform, CameraTargetOnly>,
  player: Query<&Transform, PlayerOnly>,
) {
  if let Ok(mut camera_trans) = camera.single_mut() {
    let target = if let Ok(target) = targets.single() {
      target.translation
    } else if let Ok(player_trans) = player.single() {
      player_trans.translation
    } else {
      camera_trans.translation
    };

    let dir = (target - camera_trans.translation).normalize_or_zero();
    let speed = ((target - camera_trans.translation).length() * SPRITE_SIZE as f32).min(cam_speed.0);
    
    camera_trans.translation += dir * speed * time.delta_seconds();
  }
}

/// [Plugin] for camera systems.
pub struct CameraPlugin;

/// [Res] for how fast the camera can move per frame.
pub struct CameraTrackingSpeed(pub f32);

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource::<CameraTrackingSpeed>(CameraTrackingSpeed(SPRITE_SIZE as f32  * 64.0))
      .add_system(target_camera.system());
  }
}
