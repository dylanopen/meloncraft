use bevy::ecs::component::Component;
use bevy::math::DVec3;

pub mod flags;
pub mod last;

use crate::position::flags::EntityPositionFlags;

#[derive(Component, Debug, Clone)]
pub struct EntityPosition {
    pub location: DVec3,
    pub flags: EntityPositionFlags,
}
