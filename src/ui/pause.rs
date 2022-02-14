use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use kurinji::Kurinji;

use crate::{input::{MENU, CursorCommands}, state::{game_state::TempleState, settings::{Settings, WindowSize}}, game::{player::PlayerInputCommands, physics::PhysicsCommands, sfx::AudioChannels}};

use super::settings::render_settings_menu;

#[derive(Default)]
pub struct PauseMenuState {
  pub action_held_down: bool,
  pub menu_active: bool,
}

pub fn pause_menu_buttons(
  //mut commands: Commands,
  input: Res<Kurinji>,
  mut egui_ctx: ResMut<EguiContext>,
  //window_desc: Res<WindowDescriptor>,
  mut state: Local<PauseMenuState>,
  mut cursor_commands: ResMut<CursorCommands>,
  mut player_input_commands: ResMut<PlayerInputCommands>,
  mut physics_commands: ResMut<PhysicsCommands>,
  mut channels: ResMut<AudioChannels>,
  mut settings: ResMut<Settings>,
  temple_state: Res<TempleState>,
) {
  if !temple_state.in_game() {
    return;
  }

  if input.is_action_active(MENU) && !state.menu_active && !state.action_held_down {
    state.action_held_down = true;
    state.menu_active = true;
    cursor_commands.unlock_cursor();
    player_input_commands.revoke_input();
    physics_commands.pause();
  } else if input.is_action_active(MENU) && state.menu_active && !state.action_held_down {
    cursor_commands.lock_cursor();
    player_input_commands.grant_input();
    physics_commands.resume();
    state.action_held_down = true;
    state.menu_active = false;
    settings.update_from_audio_channels(&channels);
  } else if !input.is_action_active(MENU) {
    state.action_held_down = false;
  }

  if !state.menu_active {
    return;
  }

  egui::Window::new("Pause")
  .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
  .resizable(false)
  .collapsible(false)
  .show(egui_ctx.ctx_mut(), |ui| {
    render_settings_menu(ui, channels, settings);
  });
}
