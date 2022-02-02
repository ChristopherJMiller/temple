//! Contains Plugins for modifying aspects of the rapier physics pipeline

use bevy::prelude::*;
use bevy_rapier2d::physics::TimestepMode;
use bevy_rapier2d::prelude::*;

use crate::level::config::SPRITE_SIZE;

/// Simulation steps for moving sprites.
/// Used for applying force to the player when riding a moving sprite.
/// (see [super::AttributePlugin])
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PlayerSimulationSteps {
  ApplyMoving,
  ApplyJumping,
}

/// Startup system to configure rapier physics for sprites
pub fn configure_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
  rapier_config.timestep_mode = TimestepMode::InterpolatedTimestep;
  rapier_config.scale = SPRITE_SIZE as f32;
}

/// [Plugin] for modifying Misc Rapier Physics Components.
pub struct ModifyPhysicsPlugin;

impl Plugin for ModifyPhysicsPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(configure_rapier);
  }
}
