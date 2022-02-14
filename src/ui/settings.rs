use bevy::prelude::*;

use bevy_egui::egui::{Ui, self};

use crate::{game::sfx::AudioChannels, state::settings::{Settings, WindowSize}};

fn cycle_window_dimensions(window_size: WindowSize) -> WindowSize {
  match window_size {
    WindowSize::Dim878x768 => WindowSize::Dim1028x900,
    WindowSize::Dim1028x900 => WindowSize::Dim1170x1024,
    WindowSize::Dim1170x1024 => WindowSize::Dim878x768,
  }
}

pub fn render_settings_menu(ui: &mut Ui, mut channels: ResMut<AudioChannels>, mut settings: ResMut<Settings>) {
  ui.add(egui::Slider::new(&mut channels.main_volume, 0.0..=2.0).text("Main Volume"));
  ui.add(egui::Slider::new(&mut channels.music.1, 0.0..=2.0).text("Music Volume"));
  ui.add(egui::Slider::new(&mut channels.sfx.1, 0.0..=2.0).text("SFX Volume"));
  if ui.button(format!("Window Size {}", settings.scale)).clicked() {
    settings.scale = cycle_window_dimensions(settings.scale);
    settings.save();
  }
  ui.label("Game must be restarted to set window size.");
}
