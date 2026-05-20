//! Module for struct [`EntityPosition`] and submodules related to it.

use bevy::ecs::component::Component;
use bevy::math::DVec3;

pub mod flags;
pub mod last;

use crate::position::flags::EntityPositionFlags;

/// Component for storing an entity's position in the world, as well as some extra metadata about
/// the position (see [`EntityPositionFlags`]).
///
/// You can add this component to any players or other entities that have a position, in order to
/// store their position as a component.
///
/// See the documentation for the individual fields for more information about the data stored in
/// this component.
#[derive(Component, Debug, Clone)]
pub struct EntityPosition {

    /// The entity's position in the world, in absolute coordinates.
    /// It can be thought of as a transform of the entity's feet in the world.
    /// Stored as a bevy [`DVec3`].
    pub location: DVec3,

    pub flags: EntityPositionFlags,
}
