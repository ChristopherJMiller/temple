use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use kurinji::Kurinji;

use super::settings::render_settings_menu;
use crate::game::physics::PhysicsCommands;
use crate::game::player::PlayerInputCommands;
use crate::game::sfx::AudioChannels;
use crate::input::{CursorCommands, MENU};
use crate::level::load::NextCheckpoint;
use crate::state::game_state::TempleState;
use crate::state::settings::Settings;

#[derive(Default)]
pub struct PauseMenuState {
  pub action_held_down: bool,
  pub menu_active: bool,
  pub force_unpause: bool,
}

pub fn pause_menu_buttons(
  mut commands: Commands,
  input: Res<Kurinji>,
  mut egui_ctx: ResMut<EguiContext>,
  mut state: Local<PauseMenuState>,
  mut cursor_commands: ResMut<CursorCommands>,
  mut player_input_commands: ResMut<PlayerInputCommands>,
  mut physics_commands: ResMut<PhysicsCommands>,
  channels: ResMut<AudioChannels>,
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
  } else if (input.is_action_active(MENU) && state.menu_active && !state.action_held_down) || state.force_unpause {
    cursor_commands.lock_cursor();
    player_input_commands.grant_input();
    physics_commands.resume();
    state.action_held_down = true;
    state.menu_active = false;
    state.force_unpause = false;
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
      // Basic Settings
      render_settings_menu(ui, channels, settings);

      // Accessibility
      if ui.button("Skip to Next Checkpoint").clicked() {
        commands.spawn().insert(NextCheckpoint);
        state.force_unpause = true;
      }
    });
}
