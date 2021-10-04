//! Defines a cyclically moving sprite. `moving(dir, speed, dur)`
//!
//! TODO: `speed` should be changed to `dist`, for easier usage.
//!
//! # Usage
//! `dir`: Direction of movement
//! - `0` Right
//! - `1` Right
//! - `2` Right
//! - `3` Right
//!
//! `speed`: Max velocity of the moving sprite
//!
//! `dur`: Duration of the sprite's cycle in seconds

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::sprite::SPRITE_SIZE;
use std::f32::consts::PI;

use super::{Attribute, Player};

/// Direction of sprite movement.
#[derive(Clone, Copy)]
pub enum MovingDirection {
  Right,
  Down,
  Left,
  Up,
}

impl MovingDirection {
  pub fn from_param(value: i32) -> Option<Self> {
    match value {
      0 => Some(Self::Right),
      1 => Some(Self::Down),
      2 => Some(Self::Left),
      3 => Some(Self::Up),
      _ => None,
    }
  }
}

impl Into<Vec2> for MovingDirection {
  fn into(self) -> Vec2 {
    match self {
      Self::Right => Vec2::X,
      Self::Left => -Vec2::X,
      Self::Up => Vec2::Y,
      Self::Down => -Vec2::Y,
    }
  }
}

/// `moving` attribute state.
pub struct MovingSprite {
  pub dir: MovingDirection,
  pub speed: i32,
  pub duration: i32,

  last_delta_t: f32,
  delta: f32,
  starting_position: Vec2,
  movement_vect: Vec2,
  current_time: f32,
}

impl MovingSprite {
  pub fn new(dir: MovingDirection, speed: i32, duration: i32, position: Vec2) -> Self {
    let delta = SPRITE_SIZE as f32 * speed as f32 * duration as f32;
    let vec_dir: Vec2 = dir.into();
    MovingSprite {
      dir,
      speed,
      duration,
      starting_position: position,
      movement_vect: (vec_dir * delta),
      ..MovingSprite::default()
    }
  }

  /// Increments time and recalculates [Self::delta]
  pub fn increment_time(&mut self, delta_t: f32) {
    self.last_delta_t = delta_t;
    self.current_time += delta_t;
    self.delta = 0.5 * (self.current_time + PI).cos() + 0.5;
  }

  /// Returns the position of the sprite, per current time
  pub fn get_position(&self) -> Vec2 {
    self.starting_position + self.delta * self.movement_vect
  }

  /// Calculates a impulse that is applied to the player when on the sprite, to keep them from falling off.
  /// TODO: Calculation should be done once per frame, not once per call.
  pub fn get_delta_impulse(&self) -> Vec2 {
    let delta_pos = self.get_position() - (self.starting_position + (0.5 * (self.current_time - self.last_delta_t + PI).cos() + 0.5) * self.movement_vect);
    1.25 * delta_pos / self.last_delta_t
  }
}

impl Default for MovingSprite {
  fn default() -> Self {
    Self {
      dir: MovingDirection::Right,
      speed: 0,
      duration: 0,
      starting_position: Vec2::ZERO,
      movement_vect: Vec2::ZERO,
      current_time: 0.0,
      delta: 0.0,
      last_delta_t: 0.0,
    }
  }
}

impl Attribute for MovingSprite {
  const KEY: &'static str = "moving";

  fn build(commands: &mut Commands, target: Entity, position: Vec2, params: Vec<i32>) {
    let direction_num = params.get(0).expect("Moving Sprite Attribute was not supplied parameter 0");
    let direction = MovingDirection::from_param(*direction_num).unwrap_or_else(|| panic!("Was supplied invalid moving direction of {} for moving attribute", direction_num));
    
    let speed = *params.get(1).expect("Moving Sprite Attribute was not supplied parameter 1");
    let duration = *params.get(2).expect("Moving Sprite Attribute was not supplied parameter 2");

    commands
      .entity(target)
      .insert(MovingSprite::new(direction, speed, duration, position));
  }
}

/// Simulation steps for moving sprites.
/// Used for applying force to the player when riding a moving sprite.
/// (see [super::AttributePlugin])
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum MovingAttributeSystemSteps {
  ApplyDeltaTranslation
}

/// System to move all moving sprites per change in [Time].
pub fn moving_system(time: Res<Time>, moving_sprite: Query<(&mut MovingSprite, &mut ColliderPosition)>) {
  moving_sprite.for_each_mut(|(mut moving, mut collider_position)| {
    moving.increment_time(time.delta().as_secs_f32());
    collider_position.0 = moving.get_position().into();
  }); 
}

/// Moves the [Player] if they are on top of a moving sprite.
/// TODO: Make it move all entities with a Movable Attribute instead of just the player.
pub fn move_player(mut player: Query<(&mut RigidBodyVelocity, &RigidBodyMassProps, &mut Player)>, moving_sprite: Query<(&mut MovingSprite, &mut ColliderPosition)>) {
  if let Some((mut vel, props, player_c)) = player.iter_mut().next() {
    if let Some(entity) = player_c.on_moving_entity {
      if let Ok(moving) = moving_sprite.get_component::<MovingSprite>(entity) {
        vel.apply_impulse(props, moving.get_delta_impulse().into());
      }
    }
  }
}
