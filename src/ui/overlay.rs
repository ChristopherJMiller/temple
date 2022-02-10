//! Overlay System
//! 
//! [OverlayCommands] is a resource that handles a queue of effects for fading in and out the game.
//! Queue [OverlayCommand]s to manage the effect.
//! Effect`In` is the overlay becoming more opaque, Effect`Out` is the overlay becoming more transparent.

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_egui::{EguiContext, egui::{Rgba, self, Frame}};

use crate::state::game_state::TempleState;


/// Commands that are queued using [OverlayCommands::command]
#[derive(Debug)]
pub enum OverlayCommand {
  CutIn,
  CutOut,
  FadeIn(f32),
  FadeOut(f32)
}

impl Default for OverlayCommand {
  fn default() -> Self {
    Self::CutIn
  }
}

/// Bevy Resource to manage overlay
#[derive(Default)]
pub struct OverlayCommands {
  queue: VecDeque<OverlayCommand>,
  active_command: OverlayCommand,
  action_done: bool,
  time: f32,
}

impl OverlayCommands {
  /// Queues command for overlay
  pub fn command(&mut self, action: OverlayCommand) {
    self.queue.push_back(action);
  }

  /// Returns the color to be used in the overlay, per time dt
  pub fn get_color(&mut self, dt: f32) -> Rgba {
    if self.action_done {
      if let Some(action) = self.queue.pop_front() {
        self.active_command = action;
        self.action_done = false;
        self.time = 0.0;
      }
    }

    self.time += dt;

    let alpha;

    match self.active_command {
      OverlayCommand::CutIn => {
        self.action_done = true;
        alpha = 1.0;
      },
      OverlayCommand::CutOut => {
        self.action_done = true;
        alpha = 0.0;
      },
      OverlayCommand::FadeOut(time) => {
        if time == 0.0 {
          self.action_done = true;
          alpha = 0.0;
        } else {
          alpha = 1.0 - (self.time / time).min(1.0);
          self.action_done = (self.time / time) >= 1.0;
        }
      },
      OverlayCommand::FadeIn(time) => {
        if time == 0.0 {
          self.action_done = true;
          alpha = 1.0;
        } else {
          alpha = (self.time / time).min(1.0);
          self.action_done = (self.time / time) >= 1.0;
        }
      },
    }

    return Rgba::from_black_alpha(alpha);
  }
}

pub fn handle_overlay(mut overlay: ResMut<OverlayCommands>, egui_ctx: Res<EguiContext>, time: Res<Time>, temple_state: Res<TempleState>) {
  let color = overlay.get_color(time.delta_seconds());

  if temple_state.in_game() {
    egui::CentralPanel::default().frame(Frame {
      fill: color.into(),
      ..Default::default()
    }).show(egui_ctx.ctx(), |_| {});
  }
}

#[cfg(test)]
mod tests {
  use crate::ui::overlay::*;

  #[test]
  fn test_overlay_commands() {
    let mut commands = OverlayCommands::default();

    assert_eq!(commands.get_color(1.0).a(), 1.0);

    commands.command(OverlayCommand::CutOut);
    assert_eq!(commands.get_color(1.0).a(), 0.0);
    commands.command(OverlayCommand::CutIn);
    assert_eq!(commands.get_color(1.0).a(), 1.0);

    commands.command(OverlayCommand::FadeOut(1.0));
    assert_eq!(commands.get_color(0.5).a(), 0.5);
    assert_eq!(commands.get_color(0.5).a(), 0.0);
    assert_eq!(commands.get_color(0.5).a(), 0.0);

    commands.command(OverlayCommand::FadeIn(1.0));
    assert_eq!(commands.get_color(0.5).a(), 0.5);
    assert_eq!(commands.get_color(0.5).a(), 1.0);
    assert_eq!(commands.get_color(0.5).a(), 1.0);
  }
}
