//! Handles all title screen ui elements and state.

use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::egui::ScrollArea;
use bevy_egui::{egui, EguiContext};

use super::settings::render_settings_menu;
use crate::game::sfx::AudioChannels;
use crate::game::BeginGame;
use crate::level::TotalExits;
use crate::state::game_state::{write_saves, ActiveSave, AvaliableSaves, GameSaveState};
use crate::state::settings::Settings;
use crate::util::files::from_game_root;
use crate::VERSION;

/// Tag command to load the title screen.
#[derive(Component)]
pub struct LoadTitleScreen;

/// Tag attached to all title screen related entities.
#[derive(Component)]
pub struct TitleScreen;

/// Tag to disable the title screen;
#[derive(Component)]
pub struct HideTitleScreen;

pub enum TitleMenuStates {
  MainButtons,
  SelectSaves,
  NewSavePrompt,
  Settings,
}

/// State for manging EGui Drawing
pub struct TitleMenuState {
  pub show_title_menu_egui: bool,
  pub state: TitleMenuStates,
  pub save_name_input: String,
}

impl Default for TitleMenuState {
  fn default() -> Self {
    Self {
      show_title_menu_egui: false,
      state: TitleMenuStates::MainButtons,
      save_name_input: String::default(),
    }
  }
}

/// Spawns the Bevy UI Elements for the Title Screen
fn build_title_screen(commands: &mut Commands, asset_server: &Res<AssetServer>, settings: &Res<Settings>) {
  // Game Title
  commands
    .spawn_bundle(TextBundle {
      style: Style {
        position_type: PositionType::Absolute,
        align_self: AlignSelf::FlexEnd,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        position: Rect {
          left: Val::Percent(23.0),
          right: Val::Undefined,
          top: Val::Percent(10.0),
          bottom: Val::Undefined,
        },
        ..Default::default()
      },
      text: Text {
        sections: vec![TextSection {
          value: "Temple".to_string(),
          style: TextStyle {
            font: asset_server.load(from_game_root("assets/fonts/unifont.ttf")),
            font_size: 256.0 * settings.scale.get_ui_scale() as f32,
            color: Color::WHITE,
          },
        }],
        alignment: TextAlignment {
          vertical: VerticalAlign::Center,
          horizontal: HorizontalAlign::Center,
        },
      },
      ..Default::default()
    })
    .insert(TitleScreen);

  // Version
  commands
    .spawn_bundle(TextBundle {
      style: Style {
        position_type: PositionType::Absolute,
        ..Default::default()
      },
      text: Text {
        sections: vec![TextSection {
          value: format!("Version {}", VERSION),
          style: TextStyle {
            font: asset_server.load(from_game_root("assets/fonts/unifont.ttf")),
            font_size: 32.0 * settings.scale.get_ui_scale() as f32,
            color: Color::WHITE,
          },
        }],
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(TitleScreen);
}

/// Loads the title screen via [LoadTitleScreen] if not already loaded.
pub fn setup_title_screen(
  mut commands: Commands,
  mut title_menu_state: ResMut<TitleMenuState>,
  query: Query<Entity, With<LoadTitleScreen>>,
  ensure_not_loaded_query: Query<Entity, With<TitleScreen>>,
  asset_server: Res<AssetServer>,
  settings: Res<Settings>,
) {
  if let Ok(ent) = query.get_single() {
    if ensure_not_loaded_query.iter().next().is_none() {
      build_title_screen(&mut commands, &asset_server, &settings);
      title_menu_state.show_title_menu_egui = true;
    }

    commands.entity(ent).despawn();
  }
}

/// Deletes the title screen via [HideTitleScreen]
pub fn delete_title_screen(
  mut commands: Commands,
  mut title_menu_state: ResMut<TitleMenuState>,
  tag: Query<Entity, With<HideTitleScreen>>,
  elements: Query<Entity, With<TitleScreen>>,
) {
  if let Ok(ent) = tag.get_single() {
    for ent in elements.iter() {
      commands.entity(ent).despawn();
    }

    title_menu_state.show_title_menu_egui = false;
    commands.entity(ent).despawn();
  }
}

/// EGui Coroutine for title screen
pub fn title_menu_buttons(
  mut commands: Commands,
  mut egui_ctx: ResMut<EguiContext>,
  window_desc: Res<WindowDescriptor>,
  mut title_menu_state: ResMut<TitleMenuState>,
  mut saves: ResMut<AvaliableSaves>,
  mut active_save: ResMut<ActiveSave>,
  mut exit_count_cache: Local<HashMap<String, usize>>,
  total_exits: Res<TotalExits>,
  channels: ResMut<AudioChannels>,
  settings: ResMut<Settings>,
) {
  if !title_menu_state.show_title_menu_egui {
    return;
  }

  match title_menu_state.state {
    TitleMenuStates::MainButtons => {
      egui::Area::new("Menu")
        .fixed_pos(egui::pos2((window_desc.width / 2.0) - 100.0, window_desc.height / 2.0))
        .show(egui_ctx.ctx_mut(), |ui| {
          ui.vertical(|ui| {
            if ui.add_sized([200.0, 50.0], egui::Button::new("Play")).clicked() {
              title_menu_state.state = TitleMenuStates::SelectSaves;
            }
            if ui.add_sized([200.0, 50.0], egui::Button::new("Settings")).clicked() {
              title_menu_state.state = TitleMenuStates::Settings;
            }
          });
        });
    },
    TitleMenuStates::SelectSaves => {
      egui::Area::new("Saves")
        .fixed_pos(egui::pos2((window_desc.width / 2.0) - 200.0, window_desc.height / 2.0))
        .show(egui_ctx.ctx_mut(), |ui| {
          ScrollArea::new([false; 2]).show(ui, |ui| {
            if ui.add_sized([400.0, 35.0], egui::Button::new("New Save")).clicked() {
              title_menu_state.state = TitleMenuStates::NewSavePrompt;
              title_menu_state.save_name_input = String::default();
            }

            for (name, save) in &saves.0 {
              if !exit_count_cache.contains_key(name) {
                exit_count_cache.insert(name.to_string(), save.num_cleared_exits());
              }

              let file_fmt = format!("{} {}/{}", name, exit_count_cache[name], total_exits.0);

              if ui.add_sized([400.0, 35.0], egui::Button::new(file_fmt)).clicked() {
                active_save.0 = Some(save.clone());
                commands.spawn().insert(HideTitleScreen);
                commands.spawn().insert(BeginGame);
              }
            }
          });
        });
    },
    TitleMenuStates::NewSavePrompt => {
      egui::Area::new("NewSave")
        .fixed_pos(egui::pos2((window_desc.width / 2.0) - 100.0, window_desc.height / 2.0))
        .show(egui_ctx.ctx_mut(), |ui| {
          ui.vertical(|ui| {
            ui.label("Create a New Save");
            ui.add(egui::TextEdit::singleline(&mut title_menu_state.save_name_input));
            if ui.add_sized([300.0, 35.0], egui::Button::new("Create Save")).clicked() {
              saves.0.insert(
                title_menu_state.save_name_input.clone(),
                GameSaveState::new(title_menu_state.save_name_input.clone()),
              );
              write_saves(&saves.0);
              title_menu_state.state = TitleMenuStates::SelectSaves;
            }
            if ui.add_sized([300.0, 35.0], egui::Button::new("Cancel")).clicked() {
              title_menu_state.state = TitleMenuStates::SelectSaves;
            }
          });
        });
    },
    TitleMenuStates::Settings => {
      egui::Window::new("Settings")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(egui_ctx.ctx_mut(), |ui| {
          render_settings_menu(ui, channels, settings);
          if ui.button("Back").clicked() {
            title_menu_state.state = TitleMenuStates::MainButtons;
          }
        });
    },
  }
}
