//! Contains all debug based UI elements.

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::util::cli::CliArgs;

/// Component tag for the FPS counter [Text].
pub struct FpsText;

/// Start up system for [FpsText] UI.
pub fn setup_fps_text(mut commands: Commands, cli_args: Res<CliArgs>, asset_server: Res<AssetServer>) {
  if !cli_args.show_fps_counter {
    return;
  }

  commands
    .spawn_bundle(TextBundle {
      style: Style {
        align_self: AlignSelf::FlexEnd,
        ..Default::default()
      },
      text: Text {
        sections: vec![
          TextSection {
            value: "FPS: ".to_string(),
            style: TextStyle {
              font: asset_server.load("fonts/Vollkorn-Bold.ttf"),
              font_size: 30.0,
              color: Color::WHITE,
            },
          },
          TextSection {
            value: "".to_string(),
            style: TextStyle {
              font: asset_server.load("fonts/Vollkorn-Medium.ttf"),
              font_size: 30.0,
              color: Color::GOLD,
            },
          },
        ],
        ..Default::default()
      },
      ..Default::default()
    })
    .insert(FpsText);
}

/// System that updates FPS counter via [FrameTimeDiagnosticsPlugin].
pub fn update_fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
  for mut text in query.iter_mut() {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(average) = fps.average() {
        text.sections[1].value = format!("{:.0}", average);
      }
    }
  }
}
