use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::state::EditorState;
use crate::level::config::LevelManifest;
use crate::level::LevelId;
use crate::level::util::get_level_manifests;

#[derive(Clone)]
pub struct LevelMenuItem(pub LevelId, pub LevelManifest);

impl Into<LevelMenuItem> for (u32, LevelManifest) {
  fn into(self) -> LevelMenuItem {
    LevelMenuItem(self.0, self.1)
  }
}

#[derive(Default)]
pub struct ToolbarState {
  pub show_file_menu: bool,
  pub show_open_levels_menu: bool,
  pub level_items: Vec<LevelMenuItem>,
}

fn get_level_menu_items() -> Vec<LevelMenuItem> {
  get_level_manifests().iter().map(|x| x.clone().into()).collect()
}

pub fn editor_ui(
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<ToolbarState>,
) {
  egui::Area::new("Toolbar")
    .fixed_pos(egui::pos2(10.0, 10.0))
    .show(egui_context.ctx(), |ui| {
      ui.horizontal(|ui| {
        if ui.button("File").clicked() {
          toolbar_state.show_file_menu = !toolbar_state.show_file_menu;
        }
      });
    });
}

pub fn editor_file_menu(
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<ToolbarState>,
) {
  if toolbar_state.show_file_menu {
    egui::Area::new("File")
      .fixed_pos(egui::pos2(10.0, 50.0))
      .show(egui_context.ctx(), |ui| {
        if ui.button("Open").clicked() {
          toolbar_state.show_file_menu = false;
          toolbar_state.level_items = get_level_menu_items();
          toolbar_state.show_open_levels_menu = true;
        }
      });
  }
}

/// [Plugin] for manging editor ui
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<EditorState>()
      .init_resource::<ToolbarState>()
      .add_system(editor_ui.system())
      .add_system(editor_file_menu.system());
  }
}
