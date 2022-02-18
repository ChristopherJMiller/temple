//! Manages the FSM for the player's orbs
//! 
//! # System Architecture
//! The player has a set of orbs, known as the orb cluster, following them around, one for each level cleared.
//! These orbs act with seperation and cohesion in mind.
//! The cluster can be administered a [OrbClusterCommand], which in a number of orbs required will be reserved for the affect.
//! By default, orbs in the cluster have an idle following state

use std::collections::VecDeque;

use bevy::prelude::*;
use rand::Rng;

use crate::level::util::get_texture_path;

use std::f32::consts::PI;
use super::attributes::Player;

type DashId = u8;

pub enum DashState {
  Counter,
  CrossHair,
  Cooldown
}

pub enum OrbState {
  /// Follow player, params is offset from player center (home position)
  FollowPlayer(Vec2),
  /// Become a dash orb
  Dash(DashId, DashState),
}

#[derive(Component)]
pub struct PlayerOrb {
  pub state: OrbState,
  pub vel: f32,
  pub dist: f32,
  pub accel: f32,
  pub orbit_dist: f32,
  pub orbit_speed: f32,
  pub orbit_t: f32,
}

impl Default for PlayerOrb {
  fn default() -> Self {
    let mut rng = rand::thread_rng();

    let dist: f32 = rng.gen_range(15.0..24.0);
    let angle_t: f32 = rng.gen_range(0.0..1.0);
    let angle: f32 = angle_t * (2. * PI);

    let x = dist * angle.cos();
    let y = dist * angle.sin();

    Self {
      state: OrbState::FollowPlayer([x, y].into()),
      vel: rng.gen_range(150.0..200.0),
      dist: rng.gen_range(8.0..16.0),
      accel: rng.gen_range(16.0..32.0),
      orbit_dist: dist,
      orbit_speed: rng.gen_range(8.0..16.0),
      orbit_t: angle_t,
    }
  }
}

impl PlayerOrb {
  pub fn avaliable(&self) -> bool {
    if let OrbState::FollowPlayer(_) = self.state {
      true
    } else {
      false
    }
  }
}

pub enum PlayerOrbCommand {
  SetOrbCount(usize),
  SetDashCount(usize),
}

#[derive(Default)]
pub struct PlayerOrbCommands {
  queue: VecDeque<PlayerOrbCommand>
}

impl PlayerOrbCommands {
  pub fn set_orb_count(&mut self, count: usize) {
    self.queue.push_back(PlayerOrbCommand::SetOrbCount(count));
  }

  pub fn set_dash_count(&mut self, count: usize) {
    self.queue.push_back(PlayerOrbCommand::SetDashCount(count));
  }

  pub fn pop(&mut self) -> Option<PlayerOrbCommand> {
    self.queue.pop_front()
  }
}

fn build_orb(commands: &mut Commands, asset_server: &Res<AssetServer>, player_trans: &Transform) {
  commands
    .spawn_bundle(SpriteBundle {
      texture: asset_server.load(get_texture_path(&"aspectorb.png".to_string())),
      transform: player_trans.clone(),
      ..Default::default()
    }).insert(PlayerOrb::default());
}

fn handle_orb_command_queue(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  player: Query<&Transform, With<Player>>,
  mut queue: ResMut<PlayerOrbCommands>,
  mut active_orbs: Query<(Entity, &mut PlayerOrb)>,
) {
  if let Ok(player) = player.get_single() {
    if let Some(command) = queue.pop() {
      match command {
        PlayerOrbCommand::SetOrbCount(count) => {
          let current_count = active_orbs.iter().count();
          if count > current_count {
            let diff = count - current_count;
            for _ in 0..diff {
              build_orb(&mut commands, &asset_server, player);
            }
          } else if current_count > count {
            let diff = current_count - count;
            active_orbs.iter().filter(|(_, orb)| orb.avaliable()).take(diff).for_each(|(ent, _)| commands.entity(ent).despawn());
          }
        },
        PlayerOrbCommand::SetDashCount(count) => {
          
        },
      }
    }
  }
}

fn handle_orb_state(player: Query<&Transform, (With<Player>, Without<PlayerOrb>)>, mut orbs: Query<(&mut PlayerOrb, &mut Transform), Without<Player>>, time: Res<Time>) {
  if let Ok(player_trans) = player.get_single() {
    orbs.for_each_mut(|(mut orb, mut trans)| {
      match orb.state {
        OrbState::FollowPlayer(offset) => {
          let movement_vec = (player_trans.translation.truncate() + offset) - trans.translation.truncate();
          let move_dist = movement_vec.length() / orb.dist;

          if move_dist > 10.0 {
            trans.translation = player_trans.translation.truncate().extend(trans.translation.z);
          } else {
            let accel = orb.accel * (move_dist).min(1.0);
            let vel = accel * (move_dist).powf(2.0);
            trans.translation = (trans.translation.truncate() + (movement_vec.normalize_or_zero() * vel * time.delta_seconds())).extend(trans.translation.z);
          }


          // Update orbit
          orb.orbit_t += time.delta_seconds() / orb.orbit_speed;
          if orb.orbit_t >= 1.0 {
            orb.orbit_t -= 1.0;
          }

          let angle: f32 = orb.orbit_t * (2. * PI);

          let dist_adjust = ((1.0 - (move_dist).min(1.0)) + 0.2).min(1.0);
          let x = dist_adjust * orb.orbit_dist * angle.cos();
          let y = dist_adjust * orb.orbit_dist * angle.sin();

          orb.state = OrbState::FollowPlayer([x, y].into());
        },
        OrbState::Dash(_, _) => todo!(),
      }
    });
  }
}

/// [Plugin] for handling player orbs.
pub struct PlayerOrbPlugin;

impl Plugin for PlayerOrbPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<PlayerOrbCommands>()
      .add_system(handle_orb_command_queue)
      .add_system(handle_orb_state);
  }
}

