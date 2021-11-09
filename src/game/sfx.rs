use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

/// Jump sound fx path
const JUMP_SFX: &str = "audio/sfx/jump.wav";
const CHECKPOINT_SFX: &str = "audio/sfx/checkpoint.wav";

#[derive(Default)]
pub struct SfxHandles {
  pub jump: Handle<AudioSource>,
  pub checkpoint: Handle<AudioSource>
}

fn load_sfx(  
  asset_server: Res<AssetServer>,
  mut sfx_handles: ResMut<SfxHandles>
) {
  // Load sfxs
  sfx_handles.jump = asset_server.load(JUMP_SFX);
  sfx_handles.checkpoint = asset_server.load(CHECKPOINT_SFX);
}

/// [Plugin] for sfx startup.
pub struct SfxPlugin;

impl Plugin for SfxPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<SfxHandles>()
      .add_startup_system(load_sfx.system());
  }
}
