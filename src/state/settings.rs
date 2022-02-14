use std::fmt::{Display, Formatter};
use std::fs;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::sfx::AudioChannels;
use crate::util::files::{from_game_root, SETTINGS_PATH};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum WindowSize {
  Dim878x768,
  Dim1028x900,
  Dim1170x1024,
}

impl WindowSize {
  pub fn get_ui_scale(&self) -> f64 {
    let (_, height) = self.into();
    height as f64 / 1024.0
  }
}

impl Display for WindowSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let (width, height) = self.into();
    write!(f, "{}x{}", width, height)
  }
}

impl Into<(u32, u32)> for WindowSize {
  fn into(self) -> (u32, u32) {
    (&self).into()
  }
}

impl Into<(u32, u32)> for &WindowSize {
  fn into(self) -> (u32, u32) {
    match self {
      WindowSize::Dim878x768 => (878, 768),
      WindowSize::Dim1028x900 => (1028, 900),
      WindowSize::Dim1170x1024 => (1170, 1024),
    }
  }
}

/// Settings entries
#[derive(Serialize, Deserialize)]
pub struct Settings {
  // Volume Settings
  pub main_mixer: f32,
  pub music_volume: f32,
  pub sfx_volume: f32,
  // Display Settings
  pub scale: WindowSize,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      main_mixer: 1.0,
      music_volume: 0.7,
      sfx_volume: 0.7,
      scale: WindowSize::Dim1028x900,
    }
  }
}

impl Settings {
  pub fn from_file() -> Self {
    if let Ok(file) = fs::read(from_game_root(SETTINGS_PATH)) {
      toml::from_slice::<Settings>(&file).expect("Failed to parse settings file!")
    } else {
      info!(target: "Settings::from_file", "Could not load settings file. Generating a new one...");
      let settings = Settings::default();
      settings.save();
      settings
    }
  }

  pub fn save(&self) {
    let file = toml::to_string_pretty(self);
    if let Ok(file) = file {
      if fs::write(from_game_root(SETTINGS_PATH), file).is_err() {
        warn!(target: "Settings::save", "Failed to write settings file to disk!");
      }
    } else {
      warn!(target: "Settings::save", "Failed to serialize settings file!");
    }
  }

  pub fn update_from_audio_channels(&mut self, channels: &AudioChannels) {
    self.main_mixer = channels.main_volume;
    self.music_volume = channels.music.1;
    self.sfx_volume = channels.sfx.1;
    self.save();
  }
}
