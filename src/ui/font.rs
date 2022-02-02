use std::fs::read;

use bevy::prelude::*;
use bevy_egui::egui::{self, FontData, FontDefinitions};
use bevy_egui::EguiContext;

use crate::util::files::from_game_root;

pub fn setup_egui_font(egui_ctx: ResMut<EguiContext>) {
  let ctx = &mut egui_ctx.ctx();

  let mut fonts = FontDefinitions::default();

  fonts.font_data.insert(
    "unifont".to_owned(),
    FontData::from_owned(read(from_game_root("assets/fonts/unifont.ttf")).unwrap()),
  );

  fonts
    .fonts_for_family
    .insert(egui::FontFamily::Proportional, vec!["unifont".to_owned()]);

  fonts
    .family_and_size
    .insert(egui::TextStyle::Heading, (egui::FontFamily::Proportional, 30.0));

  fonts
    .family_and_size
    .insert(egui::TextStyle::Body, (egui::FontFamily::Proportional, 22.0));

  fonts
    .family_and_size
    .insert(egui::TextStyle::Button, (egui::FontFamily::Proportional, 24.0));

  fonts
    .family_and_size
    .insert(egui::TextStyle::Small, (egui::FontFamily::Proportional, 15.0));

  ctx.set_fonts(fonts);
}
