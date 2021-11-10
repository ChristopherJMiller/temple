use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Settings {
  // Volume Settings
  main_mixer: f32,
  music_volume: f32,
  sfx_volume: f32,
}
