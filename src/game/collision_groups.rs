//! Static collision group definitions.

use bevy_rapier2d::prelude::*;

pub const NONE_GROUP: InteractionGroups = InteractionGroups::new(0, 0);
pub const PLAYER_HOVER_GROUP: InteractionGroups = InteractionGroups::new(0b1, 0b010);
pub const PLAYER_GROUP: InteractionGroups = InteractionGroups::new(0b1, 0b110);
pub const SOLID_GROUP: InteractionGroups = InteractionGroups::new(0b10, 0b11);
pub const DETECTS_PLAYER_GROUP: InteractionGroups = InteractionGroups::new(0b100, 0b01);
