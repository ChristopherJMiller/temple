//! Defines a cyclically moving sprite. `moving(dir, distance, time)`
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

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::sprite::SPRITE_SIZE;

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
  pub duration: f32,
  pub distance: f32,

  delta: f32,
  vec_dir: Vec2,
  speed: f32,
  starting_position: Vec2,
  movement_vect: Vec2,
  current_time: f32,
}

impl MovingSprite {
  pub fn new(dir: MovingDirection, distance: i32, duration: i32, position: Vec2) -> Self {
    let sprite_distance: f32 = distance as f32;
    let vec_dir: Vec2 = dir.into();
    MovingSprite {
      dir,
      duration: duration as f32,
      distance: sprite_distance,
      speed: sprite_distance / duration as f32,
      starting_position: position,
      vec_dir,
      movement_vect: (vec_dir * sprite_distance),
      ..MovingSprite::default()
    }
  }

  /// Increments time and recalculates [Self::delta]
  pub fn increment_time(&mut self, delta_t: f32) {
    self.current_time += delta_t;
    self.delta = 0.5 * (((2.0 * PI) / self.duration) * self.current_time + PI).cos() + 0.5;
  }

  /// Returns the position of the sprite, per current time
  pub fn get_position(&self) -> Vec2 {
    self.starting_position + self.get_position_delta()
  }

  pub fn get_position_delta(&self) -> Vec2 {
    self.delta * self.movement_vect
  }

  /// Calculates a impulse that is applied to the player when on the sprite, to
  /// keep them from falling off.
  pub fn get_passenger_force(&self) -> Vec2 {
    let mag = PI * (2.0 * PI * self.current_time / self.duration + (PI/12.0)).sin() / (self.duration);
    mag * self.vec_dir
  }
}

impl Default for MovingSprite {
  fn default() -> Self {
    Self {
      dir: MovingDirection::Right,
      duration: 0.0,
      distance: 0.0,
      speed: 0.0,
      vec_dir: Vec2::ZERO,
      starting_position: Vec2::ZERO,
      movement_vect: Vec2::ZERO,
      current_time: 0.0,
      delta: 0.0,
    }
  }
}

impl Attribute for MovingSprite {
  const KEY: &'static str = "moving";

  fn build(commands: &mut Commands, target: Entity, position: Vec2, params: Vec<i32>) {
    let direction_num = params
      .get(0)
      .expect("Moving Sprite Attribute was not supplied parameter 0");
    let direction = MovingDirection::from_param(*direction_num).unwrap_or_else(|| {
      panic!(
        "Was supplied invalid moving direction of {} for moving attribute",
        direction_num
      )
    });

    let distance = *params
      .get(1)
      .expect("Moving Sprite Attribute was not supplied parameter 1");
    let time = *params
      .get(2)
      .expect("Moving Sprite Attribute was not supplied parameter 2");

    commands
      .entity(target)
      .insert(ColliderPositionSync::Discrete)
      .insert(MovingSprite::new(direction, distance, time, position));
  }
}

/// Simulation steps for moving sprites.
/// Used for applying force to the player when riding a moving sprite.
/// (see [super::AttributePlugin])
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum MovingAttributeSystemSteps {
  ApplyDeltaTranslation,
}

/// System to move all moving sprites per change in [Time].
pub fn moving_system(time: Res<Time>, moving_sprite: Query<(&mut MovingSprite, &mut ColliderPosition)>) {
  moving_sprite.for_each_mut(|(mut moving, mut collider_position)| {
    moving.increment_time(time.delta().as_secs_f32());
    collider_position.0 = moving.get_position().into();
  });
}

/// Moves the [Player] if they are on top of a moving sprite.
/// TODO: Make it move all entities with a Movable Attribute instead of just the
/// player.
pub fn move_player(
  time: Res<Time>,
  mut player: Query<(&mut RigidBodyVelocity, &RigidBodyMassProps, &mut Player)>,
  moving_sprite: Query<(&mut MovingSprite, &mut ColliderPosition)>,
) {
  if let Ok((mut vel, props, player_c)) = player.single_mut() {
    if let Some(entity) = player_c.on_moving_entity {
      if let Ok(moving) = moving_sprite.get_component::<MovingSprite>(entity) {
        let force: Vector<Real> = (time.delta_seconds() * SPRITE_SIZE as f32 * moving.get_passenger_force()).into();
        vel.apply_impulse(props, force);
      }
    }
  }
}
