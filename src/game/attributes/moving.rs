use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::sprite::SPRITE_SIZE;
use std::f32::consts::PI;

use super::Attribute;

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

  pub fn to_vec2(&self) -> Vec2 {
    match self {
      Self::Right => Vec2::X,
      Self::Left => -Vec2::X,
      Self::Up => Vec2::Y,
      Self::Down => -Vec2::Y,
    }
  }
}

pub struct MovingSprite {
  pub dir: MovingDirection,
  pub speed: i32,
  pub duration: i32,

  starting_position: Vec2,
  movement_vect: Vec2,
  current_time: f32,
}

impl MovingSprite {
  pub fn new(dir: MovingDirection, speed: i32, duration: i32, position: Vec2) -> Self {
    let delta = SPRITE_SIZE as f32 * speed as f32 * duration as f32;
    MovingSprite {
      dir,
      speed,
      duration,
      starting_position: position,
      movement_vect: (dir.to_vec2() * delta),
      ..MovingSprite::default()
    }
  }

  pub fn increment_time(&mut self, delta_t: f32) {
    self.current_time += delta_t;
  }

  pub fn get_x(&self) -> f32 {
    self.starting_position.x + (0.5 * (self.current_time + PI).cos() + 0.5) * self.movement_vect.x
  }

  pub fn get_y(&self) -> f32 {
    self.starting_position.y + (0.5 * (self.current_time + PI).cos() + 0.5) * self.movement_vect.y
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

pub fn moving_system(time: Res<Time>, moving_sprite: Query<(&mut MovingSprite, &mut ColliderPosition)>) {
  moving_sprite.for_each_mut(|(mut moving, mut collider_position)| {
    moving.increment_time(time.delta().as_secs_f32());
    collider_position.0 = Vec2::new(moving.get_x(), moving.get_y()).into();
  });
}
