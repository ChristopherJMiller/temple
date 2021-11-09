//! Handles all title screen ui elements and state.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::game::BeginGame;
use crate::VERSION;

/// Tag command to load the title screen.
pub struct LoadTitleScreen;

/// Tag attached to all title screen related entities.
pub struct TitleScreen;

/// Tag to disable the title screen;
pub struct HideTitleScreen;

/// State for manging EGui Drawing
pub struct TitleMenuState {
  pub show_title_menu_buttons: bool,
}

impl Default for TitleMenuState {
  fn default() -> Self {
    Self {
      show_title_menu_buttons: false,
    }
  }
}

/// Spawns the Bevy UI Elements for the Title Screen
fn build_title_screen(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
            font: asset_server.load("fonts/Vollkorn-Bold.ttf"),
            font_size: 256.0,
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
          value: format!("{}", VERSION),
          style: TextStyle {
            font: asset_server.load("fonts/Vollkorn-Bold.ttf"),
            font_size: 32.0,
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
) {
  if let Ok(ent) = query.single() {
    if ensure_not_loaded_query.iter().next().is_none() {
      build_title_screen(&mut commands, &asset_server);
      title_menu_state.show_title_menu_buttons = true;
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
  if let Ok(ent) = tag.single() {
    for ent in elements.iter() {
      commands.entity(ent).despawn();
    }

    title_menu_state.show_title_menu_buttons = false;
    commands.entity(ent).despawn();
  }
}

/// EGui Coroutine for title screen
pub fn title_menu_buttons(
  mut commands: Commands,
  egui_ctx: Res<EguiContext>,
  window_desc: Res<WindowDescriptor>,
  title_menu_state: Res<TitleMenuState>,
) {
  if !title_menu_state.show_title_menu_buttons {
    return;
  }

  egui::Area::new("Menu")
    .fixed_pos(egui::pos2((window_desc.width / 2.0) - 100.0, window_desc.height / 2.0))
    .show(egui_ctx.ctx(), |ui| {
      ui.vertical(|ui| {
        if ui.add_sized([200.0, 50.0], egui::Button::new("Play")).clicked() {
          commands.spawn().insert(HideTitleScreen);
          commands.spawn().insert(BeginGame);
        }
      });
    });
}
