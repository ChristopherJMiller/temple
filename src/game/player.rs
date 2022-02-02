//! [Player] attribute systems
//!
//! TODO: This plugin should be moved into the same file as the [Player]
//! attribute.

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::prelude::*;
use kurinji::Kurinji;

use super::attributes::{Dash, DashCounter, DashCrosshair, MovingSprite, Player};
use super::collision_groups::*;
use super::physics::PlayerSimulationSteps;
use super::sfx::{AudioChannels, SfxHandles};
use crate::input::{DASH_DOWN, DASH_LEFT, DASH_RIGHT, DASH_UP, DOWN, JUMP, LEFT, RIGHT, SELECT, UP};
use crate::level::config::SPRITE_SIZE;
use crate::level::util::get_texture_path;

const PLAYER_MOVE_SPEED: i32 = 15;
const PLAYER_JUMP_FORCE: u32 = 10;

/// Consumes [Kurinji] inputs for player horizontal movement.
fn handle_player_movement(input: Res<Kurinji>, mut player_force: Query<&mut RigidBodyForcesComponent, With<Player>>) {
  if let Some(mut forces) = player_force.iter_mut().next() {
    let x = if input.is_action_active(RIGHT) {
      PLAYER_MOVE_SPEED as f32
    } else if input.is_action_active(LEFT) {
      -PLAYER_MOVE_SPEED as f32
    } else {
      0.0
    };

    let force: Vector<Real> = Vec2::new(x, 0.0).into();
    forces.force = force;
  }
}

/// Consumes [Kurinji] inputs for player hover height adjustments.
fn handle_height_adjust(input: Res<Kurinji>, mut player: Query<&mut Player>, dashing: Query<&Dash>) {
  if let Ok(dashing) = dashing.get_single() {
    if dashing.holding() {
      return;
    }
  }

  if let Ok(mut player_c) = player.get_single_mut() {
    let height = if input.is_action_active(UP) {
      3.0
    } else if input.is_action_active(DOWN) {
      1.0
    } else {
      2.0
    };

    player_c.height_adjust = height;
  }
}

/// Handles player hovering.
fn handle_player_hover(
  query_pipeline: Res<QueryPipeline>,
  collider_query: QueryPipelineColliderComponentsQuery,
  moving_sprite_query: Query<&MovingSprite>,
  mut player: Query<(&Transform, &mut Player, &mut RigidBodyVelocityComponent)>,
) {
  if let Some((trans, mut player_c, mut vel)) = player.iter_mut().next() {
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    let origin = Vec2::new(
      trans.translation.x / SPRITE_SIZE as f32,
      trans.translation.y / SPRITE_SIZE as f32,
    );
    let dir = Vec2::new(0.0, -1.0);

    let ray = Ray::new(origin.into(), dir.into());

    let impulse_coeff = 5.0;

    let below_ray = Ray::new((origin - Vec2::Y).into(), dir.into());

    let ground_cast = query_pipeline.cast_ray(&collider_set, &ray, Real::MAX, true, PLAYER_HOVER_GROUP, None);
    let deadly_cast = query_pipeline.cast_ray(&collider_set, &below_ray, 4.0, true, DEADLY_GROUP, None);

    if deadly_cast.is_some() && ground_cast.is_none() {
      return;
    } else if ground_cast.is_some() && deadly_cast.is_some() {
      let (_, g_toi) = ground_cast.unwrap();
      let (_, d_toi) = deadly_cast.unwrap();
      if d_toi <= g_toi {
        return;
      }
    }

    // Downwards raycast with specific collider group.
    if let Some((collided_handle, toi)) = ground_cast {
      let hit_point = ray.point_at(toi);
      let distance_vec = Vec2::new(
        origin.x - hit_point.coords.get(0).unwrap(),
        origin.y - hit_point.coords.get(1).unwrap(),
      );
      let mag = distance_vec.length();

      // Is "on ground"?
      if mag.abs() < player_c.height_adjust.abs() * 1.25 {
        player_c.outside_ground_bounds = false;
        if !player_c.jump_in_progress {
          player_c.grounded = true;
        }

        // If on ground, check if on moving platform
        if player_c.grounded {
          if player_c.on_moving_entity.is_none()
            || (player_c.on_moving_entity.is_some() && collided_handle.entity() != player_c.on_moving_entity.unwrap())
          {
            if moving_sprite_query
              .get_component::<MovingSprite>(collided_handle.entity())
              .is_ok()
            {
              player_c.on_moving_entity = Some(collided_handle.entity());
            }
          } else {
            player_c.on_moving_entity = None;
          }
        } else {
          player_c.on_moving_entity = None;
        }

        // Apply hovering force

        let height_ratio = mag / player_c.height_adjust;

        let adjust_i = impulse_coeff * (1.0 - height_ratio);

        let imp: Vector2<Real> = Vec2::new(0.0, adjust_i).into();

        vel.linvel.y = vel.linvel.y.max(imp.y);
      } else {
        player_c.outside_ground_bounds = true;
        player_c.on_moving_entity = None;
      }
    }
  }
}

/// Handles Player Slow Falling. When `JUMP` is actively being held, the player
/// falls slower.
fn handle_player_slow_fall(input: Res<Kurinji>, mut player: Query<&mut RigidBodyForcesComponent, With<Player>>) {
  if let Ok(mut forces) = player.get_single_mut() {
    if input.is_action_active(JUMP) {
      forces.gravity_scale = Player::SLOW_FALL_SPEED;
    } else {
      forces.gravity_scale = Player::NORMAL_FALL_SPEED;
    }
  }
}

/// Consumes [Kurinji] inputs for player jumping.
fn handle_player_jump(
  input: Res<Kurinji>,
  time: Res<Time>,
  mut player: Query<(&mut Player, &mut RigidBodyVelocityComponent)>,
  audio: Res<Audio>,
  sfx_handles: Res<SfxHandles>,
  channels: Res<AudioChannels>,
) {
  if let Ok((mut player_c, mut vel)) = player.get_single_mut() {
    // Start Jump
    if input.is_action_active(JUMP) && !player_c.jump_in_progress && player_c.grounded {
      player_c.jump_in_progress = true;
      player_c.grounded = false;
      player_c.jump_boost_time = Player::JUMP_BOOST_TIME;
      audio.play_in_channel(sfx_handles.jump.clone(), &channels.sfx.0);
    }

    // Apply Forces of the Jump
    if input.is_action_active(JUMP) && player_c.jump_in_progress && !player_c.grounded {
      if player_c.jump_boost_time > 0.0 {
        vel.linvel.y = PLAYER_JUMP_FORCE as f32;
        player_c.jump_boost_time = 0.0_f32.max(player_c.jump_boost_time - time.delta_seconds());
      }
    }

    if !input.is_action_active(JUMP) {
      player_c.jump_in_progress = false;
    }
  }
}

fn handle_dash(
  mut commands: Commands,
  input: Res<Kurinji>,
  time: Res<Time>,
  mut player: Query<(&Player, &mut RigidBodyVelocityComponent, &mut Dash)>,
  crosshair_spawned: Query<Entity, With<DashCrosshair>>,
  asset_server: Res<AssetServer>,
) {
  if let Ok((player, mut vel, mut dash)) = player.get_single_mut() {
    if player.grounded {
      dash.reset_charges();
    }

    if input.is_action_active(SELECT) && dash.can_dash() {
      if crosshair_spawned.get_single().is_err() {
        commands
          .spawn_bundle(SpriteBundle {
            texture: asset_server.load(get_texture_path(&"dashcross.png".to_string())),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
          })
          .insert(DashCrosshair);
      }

      let y = if input.is_action_active(DASH_UP) {
        1.0
      } else if input.is_action_active(DASH_DOWN) {
        -1.0
      } else {
        0.0
      };

      let x = if input.is_action_active(DASH_RIGHT) {
        1.0
      } else if input.is_action_active(DASH_LEFT) {
        -1.0
      } else {
        0.0
      };

      let vec = Vec2::new(x, y) * time.delta_seconds() * SPRITE_SIZE.pow(2) as f32;
      dash.hold(vec);
    } else {
      if dash.holding() {
        crosshair_spawned.for_each(|ent| commands.entity(ent).despawn());
        let vel_vec = dash.release() / 1.5;
        vel.linvel = vel_vec.into();
      }
    }
  }
}

fn handle_dash_crosshair(
  dash: Query<(&Transform, &Dash), With<Player>>,
  mut crosshair: Query<&mut Transform, (With<DashCrosshair>, Without<Player>)>,
) {
  if let Ok((trans, dash)) = dash.get_single() {
    let translation = trans.translation.clone();
    let dash_vec = dash.holding_vec().extend(0.0);
    if let Ok(mut cross_trans) = crosshair.get_single_mut() {
      cross_trans.translation = translation + dash_vec;
    }
  }
}

fn handle_dash_indictors(
  mut commands: Commands,
  dash: Query<(&Transform, &Dash), With<Player>>,
  mut dash_counter: Query<(Entity, &mut Transform, &DashCounter), Without<Player>>,
  asset_server: Res<AssetServer>,
) {
  if let Ok((trans, dash)) = dash.get_single() {
    let origin = trans.translation.clone();
    let dash_count = dash.charges();
    let mut count = 0;
    for (ent, mut trans, counter) in dash_counter.iter_mut() {
      if counter.0 > dash_count {
        commands.entity(ent).despawn();
      } else {
        count += 1;
        trans.translation = origin + Vec3::new(0.0, (SPRITE_SIZE as f32 / 1.5) + 3.0 * counter.0 as f32, 0.0);
      }
    }

    for i in count + 1..=dash_count {
      commands
        .spawn_bundle(SpriteBundle {
          texture: asset_server.load(get_texture_path(&"aspectorb.png".to_string())),
          transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
          ..Default::default()
        })
        .insert(DashCounter(i));
    }
  }
}

/// [Plugin] for player systems.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(handle_player_movement.label(PlayerSimulationSteps::ApplyMoving))
      .add_system(handle_player_hover.before(PlayerSimulationSteps::ApplyJumping))
      .add_system(handle_height_adjust)
      .add_system(handle_player_slow_fall)
      .add_system(handle_player_jump.label(PlayerSimulationSteps::ApplyJumping))
      .add_system(handle_dash)
      .add_system(handle_dash_crosshair)
      .add_system(handle_dash_indictors);
  }
}
