//! Contains Plugins for modifying aspects of the rapier physics pipeline

use std::collections::VecDeque;

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

pub enum PhysicsCommand {
  PausePhysics,
  ResumePhysics,
}

#[derive(Default)]
pub struct PhysicsCommands {
  queue: VecDeque<PhysicsCommand>,
  paused: bool,
}

impl PhysicsCommands {
  pub fn pause(&mut self) {
    self.queue.push_back(PhysicsCommand::PausePhysics);
  }

  pub fn resume(&mut self) {
    self.queue.push_back(PhysicsCommand::ResumePhysics);
  }

  pub fn pop(&mut self) -> Option<PhysicsCommand> {
    self.queue.pop_front()
  }

  pub fn paused(&self) -> bool {
    self.paused
  }
}

pub fn handle_physics_commands(mut commands: ResMut<PhysicsCommands>, mut rapier_config: ResMut<RapierConfiguration>) {
  if let Some(command) = commands.pop() {
    match command {
      PhysicsCommand::PausePhysics => {
        rapier_config.physics_pipeline_active = false;
        rapier_config.query_pipeline_active = false;
        commands.paused = true;
      },
      PhysicsCommand::ResumePhysics => {
        rapier_config.physics_pipeline_active = true;
        rapier_config.query_pipeline_active = true;
        commands.paused = false;
      },
    }
  }
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
    app
      .init_resource::<PhysicsCommands>()
      .add_startup_system(configure_rapier)
      .add_system(handle_physics_commands);
  }
}

#[cfg(test)]
mod tests {
  use bevy::prelude::*;
  use bevy_rapier2d::prelude::*;
  use super::*;

  #[test]
  fn test_physics_commands() {
    let mut world = World::default();
    let mut update_stage = SystemStage::parallel();

    // Setup Systems and Res
    world.insert_resource(PhysicsCommands::default());
    world.insert_resource(RapierConfiguration::default());
    update_stage.add_system(handle_physics_commands);

    update_stage.run(&mut world);

    // Asset Default State

    assert!(world.get_resource::<RapierConfiguration>().unwrap().physics_pipeline_active);
    assert!(world.get_resource::<RapierConfiguration>().unwrap().query_pipeline_active);
    assert!(!world.get_resource::<PhysicsCommands>().unwrap().paused());

    // Pause

    world.get_resource_mut::<PhysicsCommands>().unwrap().pause();

    update_stage.run(&mut world);

    assert!(!world.get_resource::<RapierConfiguration>().unwrap().physics_pipeline_active);
    assert!(!world.get_resource::<RapierConfiguration>().unwrap().query_pipeline_active);
    assert!(world.get_resource::<PhysicsCommands>().unwrap().paused());

    // Resume

    world.get_resource_mut::<PhysicsCommands>().unwrap().resume();

    update_stage.run(&mut world);

    assert!(world.get_resource::<RapierConfiguration>().unwrap().physics_pipeline_active);
    assert!(world.get_resource::<RapierConfiguration>().unwrap().query_pipeline_active);
    assert!(!world.get_resource::<PhysicsCommands>().unwrap().paused());
  }
}
