use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::prelude::*;
use kurinji::Kurinji;

use super::attributes::Player;
use super::collision_groups::*;
use crate::input::{DOWN, JUMP, LEFT, RIGHT, UP};
use crate::level::SPRITE_SIZE;

fn handle_player_movement(
  input: Res<Kurinji>,
  mut player_force: Query<(&mut RigidBodyVelocity, &RigidBodyMassProps), With<Player>>,
) {
  if let Some((mut vel, props)) = player_force.iter_mut().next() {
    let x_imp = if input.is_action_active(RIGHT) {
      SPRITE_SIZE as f32 * 24.0
    } else if input.is_action_active(LEFT) {
      SPRITE_SIZE as f32 * -24.0
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
  mut player: Query<(&Transform, &Player, &mut RigidBodyVelocity)>,
) {
  if let Some((trans, player_c, mut vel)) = player.iter_mut().next() {
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    let origin = Vec2::new(trans.translation.x, trans.translation.y - (SPRITE_SIZE as f32 / 2.0));
    let dir = Vec2::new(0.0, -1.0);

    let ray = Ray::new(origin.into(), dir.into());

    let impulse_coeff = 20.0;

    if let Some((_, toi)) = query_pipeline.cast_ray(&collider_set, &ray, Real::MAX, true, PLAYER_GROUP, None) {
      let hit_point = ray.point_at(toi);
      let distance_vec = Vec2::new(
        origin.x - hit_point.coords.get(0).unwrap(),
        origin.y - hit_point.coords.get(1).unwrap(),
      );
      let mag = distance_vec.length();

      if mag.abs() < player_c.height_adjust.abs() * 1.25 {
        let height_ratio = mag / player_c.height_adjust;

        let adjust_i = impulse_coeff * (1.0 - height_ratio);

        let imp: Vector2<Real> = Vec2::new(0.0, adjust_i).into();

        vel.linvel.y = vel.linvel.y.max(imp.y);
      }
    }
  }
}

fn handle_player_jump(input: Res<Kurinji>, mut player: Query<(&Transform, &Player, &mut RigidBodyVelocity)>) {
  if let Some((trans, player_c, mut vel)) = player.iter_mut().next() {
    if input.is_action_active(JUMP) {
      vel.linvel.y = 40.0;
    }
  }
}

pub struct PlayerHeightAdjust(f32);

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
