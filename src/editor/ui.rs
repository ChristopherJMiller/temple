use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::level::config::LevelManifest;
use crate::level::load::{LevelLoadComplete, LoadLevel, PreparedLevel};
use crate::level::save::SaveLevel;
use crate::level::util::get_level_manifests;
use crate::level::LevelId;

#[derive(Clone)]
pub struct LevelMenuItem(pub LevelId, pub LevelManifest);

impl Into<LevelMenuItem> for (u32, LevelManifest) {
  fn into(self) -> LevelMenuItem {
    LevelMenuItem(self.0, self.1)
  }
}

#[derive(Default)]
pub struct EditorState {
  pub show_file_menu: bool,
  pub show_open_levels_menu: bool,
  pub level_items: Vec<LevelMenuItem>,
  pub level_loaded: bool,
}

fn get_level_menu_items() -> Vec<LevelMenuItem> {
  get_level_manifests().iter().map(|x| x.clone().into()).collect()
}

/// Toolbar UI
pub fn toolbar(egui_context: Res<EguiContext>, mut toolbar_state: ResMut<EditorState>) {
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

/// File Dropdown Menu
pub fn editor_file_menu(
  mut commands: Commands,
  loaded_level_query: Query<Entity, (With<LoadLevel>, With<PreparedLevel>, With<LevelLoadComplete>)>,
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<EditorState>,
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

        if toolbar_state.level_loaded {
          if ui.button("Save").clicked() {
            if let Ok(ent) = loaded_level_query.single() {
              commands.entity(ent).insert(SaveLevel);
              toolbar_state.show_file_menu = false;
            }
          }
        }
      });
  }
}

fn format_menu_item(item: &LevelMenuItem) -> String {
  format!("ID {} | {}", item.0, item.1.name.clone())
}

/// Open Level Dialog
pub fn editor_open_menu(
  mut commands: Commands,
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<EditorState>,
) {
  if toolbar_state.show_open_levels_menu {
    egui::Area::new("Open Level")
      .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
      .show(egui_context.ctx(), |ui| {
        ui.label("Load Level");
        egui::ScrollArea::from_max_height(100.0).show(ui, |ui| {
          let items: Vec<_> = toolbar_state
            .level_items
            .iter()
            .map(|x| {
              (
                x.0,
                ui.add_sized([400.0, 35.0], egui::Button::new(format_menu_item(x)))
                  .clicked(),
              )
            })
            .collect();
          for (id, clicked) in items {
            if clicked {
              toolbar_state.show_open_levels_menu = false;
              toolbar_state.level_loaded = true;
              commands.spawn().insert(LoadLevel(id));
            }
          }
        });
      });
  }
}

/// [Plugin] for manging editor ui
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<EditorState>()
      .add_system(toolbar.system())
      .add_system(editor_file_menu.system())
      .add_system(editor_open_menu.system());
  }
}
