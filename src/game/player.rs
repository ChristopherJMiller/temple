use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::prelude::*;
use kurinji::Kurinji;

use super::attributes::Player;
use super::collision_groups::*;
use crate::input::{DOWN, JUMP, LEFT, RIGHT, UP};
use crate::sprite::SPRITE_SIZE;
use super::attributes::MovingSprite;

const PLAYER_MOVE_SPEED: i8 = 12;
const PLAYER_JUMP_FORCE: u8 = 120;

fn handle_player_movement(
  input: Res<Kurinji>,
  mut player_force: Query<(&mut RigidBodyVelocity, &RigidBodyMassProps), With<Player>>,
) {
  if let Some((mut vel, props)) = player_force.iter_mut().next() {
    let x_imp = if input.is_action_active(RIGHT) {
      SPRITE_SIZE as f32 * PLAYER_MOVE_SPEED as f32
    } else if input.is_action_active(LEFT) {
      SPRITE_SIZE as f32 * -PLAYER_MOVE_SPEED as f32
    } else {
      0.0
    };

    vel.apply_impulse(props, Vec2::new(x_imp, 0.0).into());
  }
}

fn handle_height_adjust(input: Res<Kurinji>, mut player: Query<&mut Player>) {
  if let Some(mut player_c) = player.iter_mut().next() {
    let height = if input.is_action_active(UP) {
      SPRITE_SIZE as f32 * 3.0
    } else if input.is_action_active(DOWN) {
      SPRITE_SIZE as f32 * 0.5
    } else {
      SPRITE_SIZE as f32 * 1.5
    };

    player_c.height_adjust = height;
  }
}

fn handle_player_hover(
  query_pipeline: Res<QueryPipeline>,
  collider_query: QueryPipelineColliderComponentsQuery,
  moving_sprite_query: Query<&MovingSprite>,
  mut player: Query<(&Transform, &mut Player, &mut RigidBodyVelocity)>,
) {
  if let Some((trans, mut player_c, mut vel)) = player.iter_mut().next() {
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    let origin = Vec2::new(trans.translation.x, trans.translation.y - (SPRITE_SIZE as f32 / 2.0));
    let dir = Vec2::new(0.0, -1.0);

    let ray = Ray::new(origin.into(), dir.into());

    let impulse_coeff = 20.0;

    // Downwards raycast with specific collider group.
    if let Some((collided_handle, toi)) = query_pipeline.cast_ray(&collider_set, &ray, Real::MAX, true, PLAYER_GROUP, None) {
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
          if player_c.on_moving_entity.is_none() || (player_c.on_moving_entity.is_some() && collided_handle.entity() != player_c.on_moving_entity.unwrap()) {
            if moving_sprite_query.get_component::<MovingSprite>(collided_handle.entity()).is_ok() {
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

fn handle_player_jump(input: Res<Kurinji>, mut player: Query<(&mut Player, &mut RigidBodyVelocity)>) {
  if let Some((mut player_c, mut vel)) = player.iter_mut().next() {
    if input.is_action_active(JUMP) && !player_c.jump_in_progress && player_c.grounded {
      player_c.jump_in_progress = true;
      player_c.grounded = false;

      vel.linvel.y = PLAYER_JUMP_FORCE as f32;
    }

    if !input.is_action_active(JUMP) && player_c.outside_ground_bounds {
      player_c.jump_in_progress = false;
    }
  }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system(handle_player_movement.system())
      .add_system(handle_player_hover.system())
      .add_system(handle_height_adjust.system())
      .add_system(handle_player_jump.system());
  }
}
