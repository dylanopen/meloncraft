//! Module for component struct [`CurrentChunk`].

use bevy::ecs::component::Component;
use bevy::math::IVec3;

/// Component to represent the current chunk (as `IVec3` coordinates) that an entity is in.
#[derive(Component, Debug, Clone, Copy)]
pub struct CurrentChunk {
    pub location: IVec3,
}
