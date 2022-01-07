use crate::level::config::Level;
use crate::level::LevelId;

pub struct EditorLevel(pub LevelId, pub Level);

#[derive(Default)]
pub struct EditorState {
  pub level_loaded: Option<EditorLevel>,
}
