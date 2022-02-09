//! Sets the players checkpoint. `checkpoint(optional x offset, optional y
//! offset)`

use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_rapier2d::prelude::*;

use super::lex::ParseArgumentItem;
use super::{Attribute, Player};
use crate::game::collision::{ContactQuery, ContactSubscription, PlayerContacted};
use crate::game::collision_groups::*;
use crate::game::sfx::{AudioChannels, SfxHandles};
use crate::level::config::SPRITE_SIZE;
use crate::level::load::{LevelLoadComplete, LoadLevel};
use crate::level::LevelId;
use crate::state::game_state::{write_save, ActiveSave, GameMode, GameSaveState, LevelSaveState, TempleState};

#[derive(Component)]
pub struct Checkpoint(pub Vec2);

impl Attribute for Checkpoint {
  const KEY: &'static str = "checkpoint";

  fn build(commands: &mut Commands, target: Entity, _: LevelId, position: Vec2, params: Vec<ParseArgumentItem>) {
    let player_offset = if params.len() > 0 {
      let x_offset = params.get(0);
      let y_offset = params.get(1);

      if x_offset.is_none() || y_offset.is_none() {
        panic!("Attempted to construct a checkpoint with an offset, but provided too few arguments!");
      } else {
        if let Some(ParseArgumentItem::Number(x)) = x_offset {
          if let Some(ParseArgumentItem::Number(y)) = y_offset {
            Vec2::new(*x as f32, *y as f32)
          } else {
            panic!("y coord not found!");
          }
        } else {
          panic!("x coord not found!");
        }
      }
    } else {
      Vec2::ZERO
    };

    let collider = ColliderBundle {
      position: position.into(),
      shape: ColliderShape::cuboid(0.5, 0.5).into(),
      material: ColliderMaterialComponent::default(),
      flags: ColliderFlags {
        collision_groups: DETECTS_PLAYER_GROUP,
        solver_groups: NONE_GROUP,
        active_events: ActiveEvents::CONTACT_EVENTS,
        ..Default::default()
      }
      .into(),
      ..Default::default()
    };

    commands
      .entity(target)
      .insert(Checkpoint(position + (player_offset * SPRITE_SIZE as f32)))
      .insert(ContactSubscription)
      .insert_bundle(collider)
      .insert(ColliderPositionSync::Discrete);
  }
}

/// Consumes [PlayerContacted] tags and sets the new player respawn
/// point.
pub fn on_checkpoint_system(
  mut commands: Commands,
  checkpoint_reached: ContactQuery<Checkpoint>,
  mut player: Query<&mut Player>,
  audio: Res<Audio>,
  sfx_handles: Res<SfxHandles>,
  channels: Res<AudioChannels>,
  loaded_level: Query<&LoadLevel, With<LevelLoadComplete>>,
  temple_state: Res<TempleState>,
  mut active_save: ResMut<ActiveSave>,
) {
  if let Ok(mut player) = player.get_single_mut() {
    checkpoint_reached.for_each(|(ent, checkpoint)| {
      if player.respawn_pos != checkpoint.0 {
        audio.play_in_channel(sfx_handles.checkpoint.clone(), &channels.sfx.0);
        if let Some(save) = &mut active_save.0 {
          if let Ok(level) = loaded_level.get_single() {
            if let GameMode::InLevel(level_entry) = temple_state.game_mode {
              player.respawn_level = level.0;
              player.respawn_pos = checkpoint.0;
              let key = GameSaveState::key(level_entry);
              if let Some(save) = save.level_clears.get_mut(&key) {
                save.set_checkpoint((level.0, checkpoint.0.x, checkpoint.0.y))
              } else {
                save.level_clears.insert(
                  GameSaveState::key(level_entry),
                  LevelSaveState::new_with_checkpoint((level.0, checkpoint.0.x, checkpoint.0.y)),
                );
              }

              write_save(save);
            }
          }
        }
      }

      commands.entity(ent).remove::<PlayerContacted>();
    });
  }
}
