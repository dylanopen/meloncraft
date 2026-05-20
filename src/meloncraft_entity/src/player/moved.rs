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

    /// The position that the player has moved *from*, as an [`EntityPosition`].
    ///
    /// When writing this message, you may want to query the player's current position stored in the
    /// ECS, and use that as the `old_position` field.
    ///
    /// When reading, you can use this alongside [`PlayerMoved::new_position`] to, for example,
    /// calculate the per-tick velocity of the player by subtracting `new_position - old_position`.
    pub old_position: EntityPosition,

    /// The position that the player has moved *to*, as an [`EntityPosition`].
    /// This should be the latest update you have of the player's current position.
    ///
    /// When reading, you can use this alongside [`PlayerMoved::old_position`] to, for example,
    /// calculate the per-tick velocity of the player by subtracting `new_position - old_position`.
    pub new_position: EntityPosition,
}
