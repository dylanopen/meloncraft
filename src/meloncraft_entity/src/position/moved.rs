//! Module for message struct [`EntityMoved`].

use crate::position::EntityPosition;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;

/// Message sent when an entity moves to a new position.
///
/// Stores the entity, their new position and their old position.
/// See the individual struct fields for more information about each stored field.
///
/// ## Writing
/// You should write (send) this message when an entity moves from one position to another, such as
/// when they move in the world or teleport.
///
/// ## Reading
/// You can read (receive) this message to be notified when an entity moves to a new position, and to
/// get information about the entity's old and new positions.
/// For example, you might want to write a system which listens for this message and broadcasts the
/// entity's new position to all other entities in the world, or to update the entity's position
/// component.
#[derive(Debug, Clone, Message)]
pub struct EntityMoved {
    /// The [`Entity`] representing the entity and client that moved, in the ECS.
    ///
    /// You should use this, alongside a `Query`, when reading this message if you want to get a
    /// component of this entity.
    pub entity: Entity,

    /// The position that the entity has moved *from*, as an [`EntityPosition`].
    ///
    /// When writing this message, you may want to query the entity's current position stored in the
    /// ECS, and use that as the `old_position` field.
    ///
    /// When reading, you can use this alongside [`EntityMoved::new_position`] to, for example,
    /// calculate the per-tick velocity of the entity by subtracting `new_position - old_position`.
    pub old_position: EntityPosition,

    /// The position that the entity has moved *to*, as an [`EntityPosition`].
    /// This should be the latest update you have of the entity's current position.
    ///
    /// When reading, you can use this alongside [`EntityMoved::old_position`] to, for example,
    /// calculate the per-tick velocity of the entity by subtracting `new_position - old_position`.
    pub new_position: EntityPosition,
}
