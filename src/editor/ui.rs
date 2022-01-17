use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::sprite::SelectedSprite;
use super::util::{
  format_menu_item, get_level_menu_items, get_music_files, get_sprite_table, get_sprite_texture_files,
  load_level_sprite_entries, validate_add_sprite_form, AddSpriteForm,
};
use crate::level::config::{LevelManifest, LevelSpriteEntry};
use crate::level::load::{LevelLoadComplete, LoadLevel, PreparedLevel};
use crate::level::save::SaveLevel;
use crate::level::LevelId;

#[derive(Clone)]
pub struct LevelMenuItem(pub LevelId, pub LevelManifest);

impl Into<LevelMenuItem> for (u32, LevelManifest) {
  fn into(self) -> LevelMenuItem {
    LevelMenuItem(self.0, self.1)
  }
}

pub enum AddSpriteSidebarState {
  Hide,
  SelectTexture,
  SelectAttributes,
}

impl Default for AddSpriteSidebarState {
  fn default() -> Self {
    Self::Hide
  }
}

#[derive(Default)]
pub struct EditorState {
  pub show_file_menu: bool,

  pub show_open_levels_menu: bool,
  pub level_items: Vec<LevelMenuItem>,
  pub level_loaded: bool,
  pub loaded_sprites: Vec<LevelSpriteEntry>,

  pub show_music_menu: bool,
  pub music_items: Vec<String>,

  pub show_add_sprite_menu: bool,
  pub add_sprite_form: AddSpriteForm,
  pub add_sprite_sidebar: AddSpriteSidebarState,
  pub sprite_texture_items: Vec<String>,
  pub placed_sprites: HashMap<IVec2, String>,
}

/// Toolbar UI
pub fn toolbar(egui_context: Res<EguiContext>, mut toolbar_state: ResMut<EditorState>) {
  egui::Area::new("Toolbar")
    .fixed_pos(egui::pos2(10.0, 10.0))
    .show(egui_context.ctx(), |ui| {
      ui.horizontal(|ui| {
        if ui.button("File").clicked() {
          toolbar_state.show_file_menu = !toolbar_state.show_file_menu;
        }
      });
    });
}

/// File Dropdown Menu
pub fn editor_file_menu(
  mut commands: Commands,
  loaded_level_query: Query<Entity, (With<LoadLevel>, With<PreparedLevel>, With<LevelLoadComplete>)>,
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<EditorState>,
) {
  if toolbar_state.show_file_menu {
    egui::Area::new("File")
      .fixed_pos(egui::pos2(10.0, 50.0))
      .show(egui_context.ctx(), |ui| {
        if ui.button("Open").clicked() {
          toolbar_state.show_file_menu = false;
          toolbar_state.level_items = get_level_menu_items();
          toolbar_state.show_open_levels_menu = true;
        }

        if toolbar_state.level_loaded {
          if ui.button("Save").clicked() {
            if let Ok(ent) = loaded_level_query.single() {
              commands.entity(ent).insert(SaveLevel);
              toolbar_state.show_file_menu = false;
            }
          }
        }
      });
  }
}

/// Open Level Dialog
pub fn editor_open_menu(
  mut commands: Commands,
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<EditorState>,
) {
  if toolbar_state.show_open_levels_menu {
    egui::Area::new("Open Level")
      .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
      .show(egui_context.ctx(), |ui| {
        ui.label("Load Level");
        egui::ScrollArea::from_max_height(100.0).show(ui, |ui| {
          let items: Vec<_> = toolbar_state
            .level_items
            .iter()
            .map(|x| {
              (
                x.0,
                ui.add_sized([400.0, 35.0], egui::Button::new(format_menu_item(x)))
                  .clicked(),
              )
            })
            .collect();
          for (id, clicked) in items {
            if clicked {
              toolbar_state.show_open_levels_menu = false;
              toolbar_state.level_loaded = true;
              commands.spawn().insert(LoadLevel(id));
              if let Some(sprites) = load_level_sprite_entries(id) {
                toolbar_state.loaded_sprites = sprites;
              }
              if let Some(table) = get_sprite_table(id) {
                toolbar_state.placed_sprites = table;
              }
            }
          }
        });
      });
  }
}

pub const EDITOR_ERASER_NAME: &str = "__EDITOR_ERASER";

/// Sidebar to select sprites and configure manifest while editing a level.
pub fn sidebar(
  mut selected_sprite: ResMut<SelectedSprite>,
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<EditorState>,
  mut loaded_level: Query<&mut PreparedLevel, With<LevelLoadComplete>>,
) {
  if !toolbar_state.level_loaded {
    return;
  }

  if let Ok(mut prepared_level) = loaded_level.single_mut() {
    egui::SidePanel::right("Sidebar")
      .resizable(false)
      .show(egui_context.ctx(), |ui| {
        // Level Title
        ui.horizontal(|ui| {
          ui.label("Title: ");
          ui.text_edit_singleline(&mut prepared_level.0.name);
        });

        // Level Music
        ui.horizontal(|ui| {
          ui.label(format!("Music: {}", prepared_level.0.music));
          if ui.button("Browse").clicked() {
            toolbar_state.music_items = get_music_files();
            toolbar_state.show_music_menu = true;
          };
        });

        if ui.button("Eraser").clicked() {
          selected_sprite.0 = Some(LevelSpriteEntry {
            name: EDITOR_ERASER_NAME.to_string(),
            offset: (0, 0).into(),
            texture: "eraser.png".to_string(),
            attributes: vec![],
          });
        }

        // Level Sprites
        ui.vertical_centered(|ui| {
          if ui.button("Add Sprite").clicked() {
            toolbar_state.add_sprite_form = Default::default();
            toolbar_state.show_add_sprite_menu = true;
          }

          for sprite_entry in toolbar_state.loaded_sprites.iter() {
            if ui.button(sprite_entry.name.as_str()).clicked() {
              selected_sprite.0 = Some(sprite_entry.clone());
            }
          }
        })
      });
  }
}

/// Music Selection Menu
pub fn show_music_menu(
  egui_context: Res<EguiContext>,
  mut toolbar_state: ResMut<EditorState>,
  mut loaded_level: Query<&mut PreparedLevel, With<LevelLoadComplete>>,
) {
  if !toolbar_state.level_loaded {
    return;
  }

  if toolbar_state.show_music_menu {
    egui::Area::new("Select Music")
      .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
      .show(egui_context.ctx(), |ui| {
        ui.label("Select Background Music");
        egui::ScrollArea::from_max_height(100.0).show(ui, |ui| {
          let items: Vec<_> = toolbar_state
            .music_items
            .iter()
            .map(|x| (x.clone(), ui.add_sized([400.0, 35.0], egui::Button::new(x)).clicked()))
            .collect();
          for (music, clicked) in items {
            if clicked {
              toolbar_state.show_music_menu = false;
              if let Ok(mut prepared_level) = loaded_level.single_mut() {
                prepared_level.0.music = music;
              }
            }
          }
        });
      });
  }
}

pub fn show_add_sprite_menu(egui_context: Res<EguiContext>, mut toolbar_state: ResMut<EditorState>) {
  if !toolbar_state.level_loaded {
    return;
  }

  if toolbar_state.show_add_sprite_menu {
    egui::Area::new("New Sprite")
      .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
      .show(egui_context.ctx(), |ui| {
        ui.horizontal(|ui| {
          ui.vertical(|ui| {
            ui.label("Create a new Level Sprite");
            ui.horizontal(|ui| {
              ui.label("Sprite Name");
              ui.text_edit_singleline(&mut toolbar_state.add_sprite_form.name);
            });

            ui.horizontal(|ui| {
              ui.label(format!("Texture: {}", toolbar_state.add_sprite_form.texture));
              if ui.button("Browse").clicked() {
                toolbar_state.sprite_texture_items = get_sprite_texture_files();
                toolbar_state.add_sprite_sidebar = AddSpriteSidebarState::SelectTexture;
              }
            });

            ui.horizontal(|ui| {
              ui.label("Tex Offset: ");
              ui.text_edit_singleline(&mut toolbar_state.add_sprite_form.offset[0]);
              ui.label(" x ");
              ui.text_edit_singleline(&mut toolbar_state.add_sprite_form.offset[1]);
            });

            if ui.button("Select Attributes").clicked() {
              toolbar_state.add_sprite_sidebar = AddSpriteSidebarState::SelectAttributes;
            }

            ui.horizontal(|ui| {
              if ui
                .add(egui::Button::new("Add Sprite").enabled(validate_add_sprite_form(&toolbar_state.add_sprite_form)))
                .clicked()
              {
                toolbar_state.show_add_sprite_menu = false;
                let entry = toolbar_state.add_sprite_form.clone().into();
                toolbar_state.loaded_sprites.push(entry);
              }

              if ui.button("Cancel").clicked() {
                toolbar_state.show_add_sprite_menu = false;
              }
            });
          });

          match toolbar_state.add_sprite_sidebar {
            AddSpriteSidebarState::Hide => {},
            AddSpriteSidebarState::SelectTexture => {
              ui.vertical(|ui| {
                ui.label("Select Sprite Texture");
                egui::ScrollArea::from_max_height(100.0).show(ui, |ui| {
                  let items: Vec<_> = toolbar_state
                    .sprite_texture_items
                    .iter()
                    .map(|x| (x.clone(), ui.add_sized([400.0, 35.0], egui::Button::new(x)).clicked()))
                    .collect();
                  for (texture, clicked) in items {
                    if clicked {
                      toolbar_state.add_sprite_sidebar = AddSpriteSidebarState::Hide;
                      toolbar_state.add_sprite_form.texture = texture;
                    }
                  }
                });
              });
            },
            AddSpriteSidebarState::SelectAttributes => {
              ui.vertical(|ui| {
                if ui.button("Close").clicked() {
                  toolbar_state.add_sprite_sidebar = AddSpriteSidebarState::Hide;
                }

                for (attr, selected) in toolbar_state.add_sprite_form.attributes.iter_mut() {
                  ui.checkbox(selected, attr.as_str());
                }
              });
            },
          }
        });
      });
  }
}

/// [Plugin] for manging editor ui
pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<EditorState>()
      .add_system(toolbar.system())
      .add_system(editor_file_menu.system())
      .add_system(editor_open_menu.system())
      .add_system(sidebar.system())
      .add_system(show_music_menu.system())
      .add_system(show_add_sprite_menu.system());
  }
}
