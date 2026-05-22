//! Module for structs [`GameProfile`] and [`GameProfileProperties`].

use crate::Uuid;
use bevy::prelude::Component;

/// Component storing a player's game profile, including their:
/// - UUID ([`GameProfile::uuid`])
/// - Username ([`GameProfile::username`])
///
/// This is used to store information **identifying** the player. It should be attached to a player
/// entity as soon as you identify their username and UUID.
#[derive(Component, Debug, Clone)]
pub struct GameProfile {

    /// The *universally unique identifier* ([`Uuid`]) defining the player.
    /// Every player (and in fact entity) will have a UUID.
    ///
    /// UUIDs are represented by a `u128`. See [`Uuid`] for more information.
    ///
    /// # Uniqueness
    /// This is basically guaranteed to be a completely random identifier, never seen before. This
    /// means it is safe (and recommended) to use UUIDs to represent a player in, for example,
    /// playerdata storage of a world.
    ///
    /// ## Static
    /// UUIDs are static: a player can't change it like they can their username.
    /// This means it is recommended to identify players by their [`GameProfile::uuid`] rather than
    /// by their [`GameProfile::username`].
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct GameProfileProperties {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
