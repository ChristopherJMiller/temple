use bevy_rapier2d::prelude::*;

pub static PLAYER_GROUP: InteractionGroups = InteractionGroups::new(0b1, 0b10);
pub static SOLID_GROUP: InteractionGroups = InteractionGroups::new(0b10, 0b11);
