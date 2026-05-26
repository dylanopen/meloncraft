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

    /// The player's username, as a `String`.
    /// This is the human-readable name they have chosen to go by.
    ///
    /// ## Uniqueness
    /// In online-mode, usernames are guaranteed to be unique per-account (i.e. no two Microsoft
    /// accounts can have the same Minecraft username).
    /// However, in offline-mode, the username (and by derivative, the UUID) can just be chosen by
    /// the client, and multiple users can attempt to join with the same username.
    /// As this can cause many issues, most server softwares choose to reject any connections if the
    /// username they try to log in as is already playing on the server. Or, some disconnect the
    /// existing connection and accept the new one.
    ///
    /// ## Dynamic
    /// A player can change their username, so if you use it to identify them, your server will break
    /// for players who change their name.
    /// It is highly recommended to use the [`GameProfile::uuid`] field to identify a player,
    /// instead of their username.
    pub username: String,
}

#[expect(
    missing_docs,
    reason = "This struct is currently unused, as it is optional and an empty GameProfileProperties array is acceptable."
)]
#[derive(Debug, Clone)]
pub struct GameProfileProperties {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
