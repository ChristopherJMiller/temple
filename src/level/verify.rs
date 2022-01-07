use bevy::prelude::*;
use super::util::{get_level_manifests, get_map_by_id};

/// System that loads all levels into the [LevelMap] resource to warn if any issues.
pub fn verify_level_files() {
  // Load level manifest directory
  let manifests = get_level_manifests();

  for (id, _) in manifests {
    let map = get_map_by_id(id);
    if map.is_none() {
      warn!(target: "verify_level_files", "Warning! Level with id {:?} does not have an associated level map. This will crash the game if you attempt to load the level!", id);
    }
  }
}
