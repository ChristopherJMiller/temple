//! Game mechanisms and sprite attribute definitions.

use attributes::AttributePlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use physics::ModifyPhysicsPlugin;
use player::PlayerPlugin;

use self::credits::CreditsPlugin;
use self::orbs::PlayerOrbPlugin;
use self::sfx::SfxPlugin;
use crate::level::load::LoadLevel;
use crate::state::game_state::{ActiveSave, GameMode, GameSaveState, TempleState};
use crate::ui::overlay::{OverlayCommand, OverlayCommands};
use crate::util::settings::{GameFile, LevelTransistionType};

pub mod attributes;
pub mod camera;
pub mod collision;
pub mod collision_groups;
pub mod credits;
pub mod physics;
pub mod player;
pub mod sfx;
pub mod orbs;

/// Command to begin the game
#[derive(Component)]
pub struct BeginGame;

/// Begins the game by command [BeginGame]. Uses the options in [GameFile] to
/// determine what to load.
fn bootstrap_game(
  mut commands: Commands,
  query: Query<Entity, With<BeginGame>>,
  active_save: Res<ActiveSave>,
  game_file: Res<GameFile>,
  mut temple_state: ResMut<TempleState>,
  mut overlay_commands: ResMut<OverlayCommands>,
) {
  if let Ok(ent) = query.get_single() {
    overlay_commands.command(OverlayCommand::CutIn);
    match game_file.level_transistion {
      LevelTransistionType::Overworld => panic!("Temple does not support overworlds yet"),
      LevelTransistionType::NoOverworld => {
        if let Some(level_order) = &game_file.level_order {
          let mut level = *level_order.first().unwrap();

          // If an active save is avaliable,
          if let Some(save) = &active_save.0 {
            // Per level in order,
            for next_level in level_order.iter() {
              // Check if a save is avaliable
              if let Some(save) = save.level_clears.get(&GameSaveState::key(*next_level)) {
                // If there is, and nothing is cleared on it, play this level.
                if !save.an_exit_cleared() {
                  level = *next_level;
                  break;
                }
              // If no save is available, this is the next level in the order.
              } else {
                level = *next_level;
                break;
              }
            }
          }

          temple_state.game_mode = GameMode::InLevel(level);
          commands.spawn().insert(LoadLevel(level));
        } else {
          panic!("Failed to start game, no level order defined");
        }
      },
    }
    commands.entity(ent).despawn();
  }
}

/// [Plugin] for handling game bootstrapping
struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(bootstrap_game);
  }
}

/// [PluginGroup] for game modules.
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut PluginGroupBuilder) {
    group
      .add(CameraPlugin)
      .add(PlayerPlugin)
      .add(AttributePlugin)
      .add(CollisionPlugin)
      .add(ModifyPhysicsPlugin)
      .add(BootstrapPlugin)
      .add(SfxPlugin)
      .add(CreditsPlugin)
      .add(PlayerOrbPlugin);
  }
}
