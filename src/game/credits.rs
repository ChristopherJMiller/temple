use bevy::{prelude::*, asset::LoadState};
use bevy_egui::{EguiContext, egui};
use kurinji::Kurinji;
use pulldown_cmark::{Tag, Event, Parser, Options, HeadingLevel};

use crate::{util::{settings::GameFile, files::{from_game_root, MUSIC_DIR_PATH}}, input::SELECT};
use bevy_kira_audio::{Audio, AudioSource};

use super::sfx::AudioChannels;

#[derive(Debug, Clone, Copy)]
pub enum TextStyle {
  Bold,
  Italics,
}

#[derive(Debug, Clone)]
pub enum CreditObject {
  Header(HeadingLevel, String),
  Text(Option<TextStyle>, String),
  HLine,
  Break,
}

#[derive(Default)]
pub struct CreditsData {
  scroll_speed: f32,
  y_offset: f32,
  run_credits: bool,
  credits_data: Vec<CreditObject>,
}

pub fn set_scroll_speed (
  mut credits_data: ResMut<CreditsData>,
  input: Res<Kurinji>,
) {
  if credits_data.run_credits {
    if input.is_action_active(SELECT) {
      credits_data.scroll_speed = 150.0;
    } else {
      credits_data.scroll_speed = 20.0;
    }
  }
}

pub fn load_credits(
  mut commands: Commands,
  play_credits: Query<Entity, With<PlayCredits>>,
  game_file: Res<GameFile>,
  asset_server: Res<AssetServer>,
  mut credits_data: ResMut<CreditsData>,
  audio: Res<Audio>,
  channels: Res<AudioChannels>,
) {
  if let Ok(ent) = play_credits.single() {
    let music_path = from_game_root(MUSIC_DIR_PATH).join(game_file.credit_music.clone());
    let music: Handle<AudioSource> = asset_server.get_handle(music_path.clone().into_os_string().to_str().unwrap());

    // Load music
    if asset_server.get_load_state(&music) == LoadState::Loaded {
      audio.stop_channel(&channels.music.0);
      audio.play_looped_in_channel(
        asset_server.load(music_path.into_os_string().to_str().unwrap()),
        &channels.music.0,
      );
    } else if asset_server.get_load_state(&music) != LoadState::Loading {
      let _: Handle<AudioSource> = asset_server.load(music_path.into_os_string().to_str().unwrap());
      return;
    } else {
      // Wait for load
      return;
    }

    // Build credits text
    let parser = Parser::new_ext(game_file.credits.as_str(), Options::empty());

    let mut heading_level: Option<HeadingLevel> = None;
    let mut text_style: Option<TextStyle> = None;

    let mut credits = Vec::new();
    for event in parser {
      match event {
        Event::Start(tag) => match tag {
            Tag::Heading(lvl, _, _) => heading_level = Some(lvl),
            Tag::Emphasis => text_style = Some(TextStyle::Italics),
            Tag::Strong => text_style = Some(TextStyle::Bold),
            _ => (),
        },
        Event::End(tag) => match tag {
            Tag::Heading(_, _, _) => heading_level = None,
            Tag::Emphasis => text_style = None,
            Tag::Strong => text_style = None,
            _ => (),
        },
        Event::Text(text) => if let Some(lvl) = heading_level {
          credits.push(CreditObject::Header(lvl, text.to_string()))
        } else {
          credits.push(CreditObject::Text(text_style, text.to_string()))
        },
        Event::SoftBreak | Event::HardBreak => credits.push(CreditObject::Break),
        Event::Rule => credits.push(CreditObject::HLine),
        _ => (),
      }
    }

    credits_data.credits_data = credits;
    credits_data.run_credits = true;


    commands.entity(ent).despawn();
  }
}

pub fn run_credits(
  egui_context: Res<EguiContext>,
  mut credits: ResMut<CreditsData>,
  time: Res<Time>,
  window_desc: Res<WindowDescriptor>,
) {
  if credits.run_credits {
    egui::Area::new("Credits")
    .anchor(egui::Align2::CENTER_BOTTOM, [0.0, credits.y_offset.round()])
    .show(egui_context.ctx(), |ui| {
      ui.set_width(window_desc.width * 0.66);
      ui.vertical_centered(|ui| {
        for element in credits.credits_data.iter() {
          match element {
            CreditObject::Header(_, text) => { ui.heading(text.as_str()); },
            CreditObject::Text(_, text) => { 
              ui.label(text.as_str());
            },
            CreditObject::HLine => { ui.separator(); },
            CreditObject::Break => (),
          }
        }
      });
    });

    credits.y_offset -= time.delta_seconds() * credits.scroll_speed;
  }
}

/// Command to play credits. Does not unload current level.
pub struct PlayCredits;

/// [Plugin] for handling credits.
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<CreditsData>()
      .add_system(load_credits.system())
      .add_system(set_scroll_speed.system())
      .add_system(run_credits.system());
  }
}
