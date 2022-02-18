//! Manages the FSM for the player's orbs
//! 
//! # System Architecture
//! The player has a set of orbs, known as the orb cluster, following them around, one for each level cleared.
//! These orbs act with seperation and cohesion in mind.
//! The cluster can be administered a [OrbClusterCommand], which in a number of orbs required will be reserved for the affect.
//! By default, orbs in the cluster have an idle following state

use std::collections::VecDeque;

use bevy::{prelude::*, asset::AssetPath};
use rand::Rng;

use crate::level::{util::get_texture_path, config::SPRITE_SIZE};

use std::f32::consts::PI;
use super::attributes::{Player, DashCrosshair};

type DashId = usize;

#[derive(Debug, Clone)]
pub enum DashState {
  Counter,
  CrossHair
}

#[derive(Debug, Clone)]
pub enum OrbState {
  /// Follow player, params is offset from player center (home position)
  FollowPlayer(Vec2),
  /// Become a dash orb
  Dash(DashId, DashState),
}

impl OrbState {
  pub fn variant_eq(&self, b: &Self) -> bool {
    match (self, b) {
        (OrbState::FollowPlayer(_), OrbState::FollowPlayer(_)) => true,
        (OrbState::Dash(_, _), OrbState::Dash(_, _)) => true,
        _ => false
    }
  }
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
  pub last_textured_state: OrbState,
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
      last_textured_state: OrbState::FollowPlayer([x, y].into()),
    }
  }
}

impl PlayerOrb {
  pub fn from_state(state: OrbState) -> Self {
    let mut this = Self::default();
    this.state = state;
    this
  }

  pub fn get_dash_id(&self) -> Option<DashId> {
    if let OrbState::Dash(id, DashState::Counter) = self.state {
      Some(id)
    } else {
      None
    }
  }

  pub fn is_dash_counter(&self) -> bool {
    if let OrbState::Dash(_, DashState::Counter) = self.state {
      true
    } else {
      false
    }
  }

  pub fn is_dash_crosshair(&self) -> bool {
    if let OrbState::Dash(_, DashState::CrossHair) = self.state {
      true
    } else {
      false
    }
  }

  pub fn is_dash_orb(&self) -> bool {
    if let OrbState::Dash(_, _) = self.state {
      true
    } else {
      false
    }
  }

  pub fn avaliable(&self) -> bool {
    if let OrbState::FollowPlayer(_) = self.state {
      true
    } else {
      false
    }
  }

  pub fn get_texture_path_by_state<'a>(&self) -> AssetPath<'a> {
    match self.state {
      OrbState::FollowPlayer(_) => get_texture_path(&"aspectorb.png".to_string()),
      OrbState::Dash(_, _) => get_texture_path(&"dashorb.png".to_string()),
    }
  }
}

#[derive(Debug)]
pub enum PlayerOrbCommand {
  SetOrbCount(usize),
  SetDashCount(usize),
  UseDashCount,
  AllocCrosshair,
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

  pub fn use_dash_count(&mut self) {
    self.queue.push_back(PlayerOrbCommand::UseDashCount);
  }

  pub fn alloc_crosshair(&mut self) {
    self.queue.push_back(PlayerOrbCommand::AllocCrosshair);
  }

  pub fn pop(&mut self) -> Option<PlayerOrbCommand> {
    self.queue.pop_front()
  }
}

fn handle_orb_sprite(
  mut commands: Commands, 
  asset_server: Res<AssetServer>, 
  player: Query<&Transform, With<Player>>,
  orbs_to_create: Query<(Entity, &PlayerOrb), Without<Handle<Image>>>, 
  mut changed_orbs: Query<(&mut Handle<Image>, &mut PlayerOrb), Changed<PlayerOrb>>
) {
  if let Ok(player_trans) = player.get_single() {
    orbs_to_create.for_each(|(ent, orb)| {
      commands.entity(ent).insert_bundle(SpriteBundle {
        texture: asset_server.load(orb.get_texture_path_by_state()),
        transform: player_trans.clone(),
        ..Default::default()
      });
    });
  }

  changed_orbs.for_each_mut(|(mut image_handle, mut orb)| {
    if !orb.state.variant_eq(&orb.last_textured_state) {
      *image_handle = asset_server.load(orb.get_texture_path_by_state());
      orb.last_textured_state = orb.state.clone();
    }
    
  });
}

fn handle_orb_command_queue(
  mut commands: Commands,
  mut queue: ResMut<PlayerOrbCommands>,
  mut active_orbs: Query<(Entity, &mut PlayerOrb)>,
) {
  if let Some(command) = queue.pop() {
    match command {
      PlayerOrbCommand::SetOrbCount(count) => {
        let current_count = active_orbs.iter().count();
        if count > current_count {
          let diff = count - current_count;
          for _ in 0..diff {
            commands.spawn().insert(PlayerOrb::default());
          }
        } else if current_count > count {
          let diff = current_count - count;
          active_orbs.iter().filter(|(_, orb)| orb.avaliable()).take(diff).for_each(|(ent, _)| commands.entity(ent).despawn());
        }
      },
      PlayerOrbCommand::SetDashCount(count) => {
        // Find current number of dash orbs
        let active_dash_count = active_orbs.iter().filter(|(_, orb)| orb.is_dash_orb()).count();

        // If more set, allocate more dash orbs
        if count > active_dash_count {
          let diff = count - active_dash_count;

          // Ensure there is enough avaliable orbs
          let avaliable_orbs = active_orbs.iter().filter(|(_, orb)| orb.avaliable()).count();

          // Allocate whatever is avaliable
          let allocate_count = avaliable_orbs.min(diff);
          active_orbs.iter_mut().filter(|(_, orb)| orb.avaliable()).take(allocate_count).enumerate().for_each(|(i, (_, mut orb))| {
            orb.state = OrbState::Dash(active_dash_count + i, DashState::Counter);
          });

          // Check if remainder to allocate, if so spawn that number in as allocated.
          if diff > avaliable_orbs {
            let to_spawn = diff - avaliable_orbs;
            for i in 0..to_spawn {
              commands.spawn().insert(PlayerOrb::from_state(OrbState::Dash(active_dash_count + avaliable_orbs + i, DashState::Counter)));
            }
          }
        }
      },
      PlayerOrbCommand::UseDashCount => {
        // Attempt to find crosshair first
        if let Some((_, mut orb)) = active_orbs.iter_mut().filter(|(_, orb)| orb.is_dash_crosshair()).next() {
          orb.state = PlayerOrb::default().state;
        } else {
          // If no crosshair, default to a dash counter
          let orb = active_orbs.iter_mut().filter(|(_, orb)| orb.is_dash_counter()).max_by(|x, y| x.1.get_dash_id().unwrap().cmp(&y.1.get_dash_id().unwrap()));
          if let Some((_, mut orb)) = orb {
            orb.state = PlayerOrb::default().state;
          }
        }
      },
      PlayerOrbCommand::AllocCrosshair => {
        // Return if crosshair is already allocated
        if active_orbs.iter_mut().filter(|(_, orb)| orb.is_dash_crosshair()).next().is_some() {
          return;
        }

        let orb = active_orbs.iter_mut().filter(|(_, orb)| orb.is_dash_counter()).max_by(|x, y| x.1.get_dash_id().unwrap().cmp(&y.1.get_dash_id().unwrap()));
        if let Some((_, mut orb)) = orb {
          orb.state = OrbState::Dash(orb.get_dash_id().unwrap(), DashState::CrossHair);
        }
      },
    }
  }
}

fn move_orb_tick(orb_trans: &Vec2, target: &Vec2, orb: &PlayerOrb, time: &Res<Time>) -> Vec2 {
  let movement_vec = target.clone() - orb_trans.clone();
  let move_dist = movement_vec.length() / orb.dist;

  if move_dist > 10.0 {
    return target.clone();
  } else {
    let accel = orb.accel * (move_dist).min(1.0);
    let vel = accel * (move_dist).powf(2.0);
    return orb_trans.clone() + (movement_vec.normalize_or_zero() * vel * time.delta_seconds());
  }
}

fn handle_orb_state(
  player: Query<&Transform, (With<Player>, Without<PlayerOrb>, Without<DashCrosshair>)>, 
  mut orbs: Query<(&mut PlayerOrb, &mut Transform), (Without<DashCrosshair>, Without<Player>)>, 
  crosshair: Query<&Transform, (With<DashCrosshair>, Without<PlayerOrb>, Without<Player>)>,
  time: Res<Time>
) {
  if let Ok(player_trans) = player.get_single() {
    orbs.for_each_mut(|(mut orb, mut trans)| {
      match &orb.state {
        OrbState::FollowPlayer(offset) => {
          trans.translation = move_orb_tick(&trans.translation.truncate(), &(player_trans.translation.truncate() + offset.clone()), &orb, &time).extend(trans.translation.z);
          let movement_vec = (player_trans.translation.truncate() + offset.clone()) - trans.translation.truncate();
          let move_dist = movement_vec.length() / orb.dist;

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
        OrbState::Dash(id, state) => match state {
          DashState::Counter => {
            let target = player_trans.translation + Vec3::new(0.0, (SPRITE_SIZE as f32 / 1.5) + 3.0 * *id as f32, 0.0);
            trans.translation = move_orb_tick(&trans.translation.truncate(), &target.truncate(), &orb, &time).extend(trans.translation.z);
          },
          DashState::CrossHair => {
            if let Ok(cross_trans) = crosshair.get_single() {
              trans.translation = move_orb_tick(&trans.translation.truncate(), &cross_trans.translation.truncate(), &orb, &time).extend(trans.translation.z);
            } else {
              let target = trans.translation + Vec3::new(0.0, (SPRITE_SIZE as f32 / 1.5) + 3.0 * *id as f32, 0.0);
              trans.translation = move_orb_tick(&trans.translation.truncate(), &target.truncate(), &orb, &time).extend(trans.translation.z);
            }
          },
        },
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
      .add_system(handle_orb_state)
      .add_system(handle_orb_sprite);
  }
}

#[cfg(test)]
mod tests {
  use bevy::prelude::*;
  use super::*;

  #[test]
  fn test_orb_system() {
    let mut world = World::default();
    let mut update_stage = SystemStage::parallel();

    // Setup Systems and Res
    world.insert_resource(PlayerOrbCommands::default());
    world.insert_resource(Time::default());
    update_stage.add_system(handle_orb_command_queue);
    update_stage.add_system(handle_orb_state);

    update_stage.run(&mut world);

    // Asset Default State
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).count(), 0);

    // Spawn orbs
    world.get_resource_mut::<PlayerOrbCommands>().unwrap().set_orb_count(4);
    update_stage.run(&mut world);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).count(), 4);

    // Allocate some to dashes
    world.get_resource_mut::<PlayerOrbCommands>().unwrap().set_dash_count(2);
    update_stage.run(&mut world);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).count(), 4);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.is_dash_counter()).count(), 2);

    // Allocate a crosshair
    world.get_resource_mut::<PlayerOrbCommands>().unwrap().alloc_crosshair();
    update_stage.run(&mut world);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).count(), 4);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.avaliable()).count(), 2);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.is_dash_counter()).count(), 1);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.is_dash_crosshair()).count(), 1);

    // Consume crosshair orb
    world.get_resource_mut::<PlayerOrbCommands>().unwrap().use_dash_count();
    update_stage.run(&mut world);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).count(), 4);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.avaliable()).count(), 3);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.is_dash_counter()).count(), 1);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.is_dash_crosshair()).count(), 0);

    // Despawn unused orbs
    world.get_resource_mut::<PlayerOrbCommands>().unwrap().set_orb_count(1);
    update_stage.run(&mut world);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.avaliable()).count(), 0);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.is_dash_counter()).count(), 1);
    assert_eq!(world.query::<&PlayerOrb>().iter(&world).filter(|x| x.is_dash_crosshair()).count(), 0);
  }
}
