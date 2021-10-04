//! Static collision group definitions.

use bevy_rapier2d::prelude::*;

pub const PLAYER_GROUP: InteractionGroups = InteractionGroups::new(0b1, 0b10);
pub const SOLID_GROUP: InteractionGroups = InteractionGroups::new(0b10, 0b11);
