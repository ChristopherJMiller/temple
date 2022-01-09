use bevy::prelude::*;

use super::util::{get_level_manifests, get_map_by_id};
use crate::state::game_state::TempleState;

/// System that loads all levels into the [LevelMap] resource to warn if any
/// issues.
pub fn verify_level_files(temple_state: Res<TempleState>) {
  // Load level manifest directory
  let manifests = get_level_manifests();

  for (id, _) in manifests {
    let map = get_map_by_id(id);
    // If in play mode, loading a level without a map will crash
    if map.is_none() && !temple_state.in_edit_mode() {
      warn!(target: "verify_level_files", "Warning! Level with id {:?} does not have an associated level map. This will crash the game if you attempt to load the level!", id);
    }
  }
}
