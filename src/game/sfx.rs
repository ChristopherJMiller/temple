use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};

use crate::util::files::from_game_root;

/// Jump sound fx path
const JUMP_SFX: &str = "assets/audio/sfx/jump.wav";
const CHECKPOINT_SFX: &str = "assets/audio/sfx/checkpoint.wav";

pub struct ChannelState(pub AudioChannel, pub f32);

impl Default for ChannelState {
  fn default() -> Self {
    Self(Default::default(), 0.7)
  }
}
pub struct AudioChannels {
  pub main_volume: f32,
  pub music: ChannelState,
  pub sfx: ChannelState,
}

impl Default for AudioChannels {
  fn default() -> Self {
    Self {
      main_volume: 1.0,
      music: Default::default(),
      sfx: Default::default(),
    }
  }
}

#[derive(Default)]
pub struct SfxHandles {
  pub jump: Handle<AudioSource>,
  pub checkpoint: Handle<AudioSource>,
}

fn load_sfx(asset_server: Res<AssetServer>, mut sfx_handles: ResMut<SfxHandles>) {
  // Load sfxs
  sfx_handles.jump = asset_server.load(from_game_root(JUMP_SFX));
  sfx_handles.checkpoint = asset_server.load(from_game_root(CHECKPOINT_SFX));
}

fn init_channel_volume(audio: Res<Audio>, channels: Res<AudioChannels>) {
  audio.set_volume_in_channel(channels.main_volume * channels.music.1, &channels.music.0);
  audio.set_volume_in_channel(channels.main_volume * channels.sfx.1, &channels.sfx.0);
}

/// [Plugin] for sfx startup.
pub struct SfxPlugin;

impl Plugin for SfxPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<AudioChannels>()
      .init_resource::<SfxHandles>()
      .add_startup_system(init_channel_volume.system())
      .add_startup_system(load_sfx.system());
  }
}
