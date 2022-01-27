//! Grants the player a dash. `dash()`

use std::mem;

use bevy::prelude::*;

use super::lex::ParseArgumentItem;
use super::Attribute;
use crate::level::LevelId;
use crate::level::config::SPRITE_SIZE;

pub struct Dash {
  charges: u32,
  capacity: u32,
  holding: bool,
  hold_vector: Vec2,
}

impl Default for Dash {
  fn default() -> Self {
    const STARTING_CAP: u32 = 1;
    Self { 
      charges: STARTING_CAP, 
      capacity: STARTING_CAP, 
      holding: Default::default(), 
      hold_vector: Default::default() 
    }
  }
}

const MAX_DIST_SQUARED: f32 = 3.0 * SPRITE_SIZE as f32;

pub struct DashCrosshair;

pub struct DashCounter(pub u32);

impl Dash {
  pub fn can_dash(&self) -> bool {
    self.charges > 0
  }

  pub fn holding(&self) -> bool {
    self.holding
  }

  pub fn hold(&mut self, vec: Vec2) {
    self.holding = true;

    let mut new_vec = self.hold_vector + vec;
    if new_vec.distance_squared(Vec2::ZERO).abs() >= MAX_DIST_SQUARED.powf(2.0) {
      new_vec = new_vec.normalize() * MAX_DIST_SQUARED;
    }
    self.hold_vector = new_vec;
  }

  pub fn holding_vec(&self) -> Vec2 {
    self.hold_vector
  }

  pub fn release(&mut self) -> Vec2 {
    self.charges = (self.charges as i32 - 1).max(0) as u32;
    self.holding = false;
    let mut result = Vec2::default();
    mem::swap(&mut self.hold_vector, &mut result);
    result
  }

  pub fn reset_charges(&mut self) {
    self.charges = self.capacity;
  }

  #[allow(dead_code)]
  pub fn set_cap(&mut self, cap: u32) {
    self.capacity = cap;
  }

  pub fn charges(&self) -> u32 {
    self.charges
  }
}

impl Attribute for Dash {
  const KEY: &'static str = "dash";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, _: Vec2, _: Vec<ParseArgumentItem>) {
    commands
      .entity(target)
      .insert(Dash::default());
  }
}
