//! Module for message struct [`EntityMoved`].

use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use crate::position::EntityPosition;

/// Message sent when an entity moves to a new position.
///
/// Stores the entity, their new position and their old position.
/// See the individual struct fields for more information about each stored field.
///
/// ## Writing
/// You should write (send) this message when a player or other entity is teleported from one
/// position to another.
///
/// ## Reading
/// You may want to read this message in order to forward a new `EntityMoved` message, so that the
/// movement is read by other systems that listen for `EntityMoved` messages, e.g. to update the
/// entity's position in the ECS.
#[derive(Debug, Clone, Message)]
pub struct TeleportEntity {

    /// The [`Entity`] representing the entity and client that should be teleported, in the ECS.
    ///
    /// You should use this, alongside a `Query`, when reading this message if you want to get a
    /// component of this entity.
    pub entity: Entity,

    /// The position that the entity has teleported *to*, as an [`EntityPosition`].
    /// This should be the latest update you have of the entity's current position.
    pub new_position: EntityPosition,
}
