pub struct Solid;

pub struct Player {
  pub height_adjust: f32,
}

impl Default for Player {
  fn default() -> Player {
    Player { height_adjust: 0.25 }
  }
}
