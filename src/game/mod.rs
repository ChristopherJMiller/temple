//! Game mechanisms and sprite attribute definitions.

use attributes::AttributePlugin;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use physics::ModifyPhysicsPlugin;
use player::PlayerPlugin;

use self::sfx::SfxPlugin;
use crate::level::load::LoadLevel;
use crate::util::settings::{GameFile, LevelTransistionType};

pub mod attributes;
pub mod camera;
pub mod collision;
pub mod collision_groups;
pub mod physics;
pub mod player;
pub mod sfx;

/// Command to begin the game
pub struct BeginGame;

/// Begins the game by command [BeginGame]. Uses the options in [GameFile] to
/// determine what to load.
fn bootstrap_game(mut commands: Commands, query: Query<Entity, With<BeginGame>>, game_file: Res<GameFile>) {
  if let Ok(ent) = query.single() {
    match game_file.level_transistion {
      LevelTransistionType::Overworld => panic!("Temple does not support overworlds yet"),
      LevelTransistionType::NoOverworld => {
        if let Some(level_order) = &game_file.level_order {
          commands.spawn().insert(LoadLevel(*level_order.first().unwrap()));
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
  fn build(&self, app: &mut AppBuilder) {
    app.add_system(bootstrap_game.system());
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
      .add(SfxPlugin);
  }
}
