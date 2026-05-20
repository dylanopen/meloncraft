//! Module for message struct [`PlayerMoved`].

use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use crate::position::EntityPosition;

/// Message sent when a player moves to a new position.
///
/// Stores the entity, their new position and their old position.
/// See the individual struct fields for more information about each stored field.
///
/// ## Writing
/// You should write (send) this message when a player moves from one position to another, such as
/// when they move in the world or teleport.
///
/// ## Reading
/// You can read (receive) this message to be notified when a player moves to a new position, and to
/// get information about the player's old and new positions.
/// For example, you might want to write a system which listens for this message and broadcasts the
/// player's new position to all other players in the world, or to update the player's position
/// component.
#[derive(Debug, Clone, Message)]
pub struct PlayerMoved {

    /// The [`Entity`] representing the player and client that moved, in the ECS.
    ///
    /// You should use this, alongside a `Query`, when reading this message if you want to get a
    /// component of this player.
    pub entity: Entity,

    pub old_position: EntityPosition,

    pub new_position: EntityPosition,
}
