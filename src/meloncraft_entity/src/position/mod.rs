//! Module for struct [`EntityPosition`] and submodules related to it.

use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::math::DVec3;

pub mod current_chunk;
pub mod flags;
pub mod last;
pub mod moved;
pub mod teleport;

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

    /// The entity which represents the world the entity is currently in.
    pub world: Entity,

    /// Extra metadata about the entity's position, such as whether the entity is on the ground or
    /// pushing against a wall. See [`EntityPositionFlags`] for more information about the flags
    /// stored in this field.
    ///
    /// May be converted to a `u8` bitset for serialization: this is explained in more detail in the
    /// documentation for [`EntityPositionFlags`].
    pub flags: EntityPositionFlags,
}
