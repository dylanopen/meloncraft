//! Module for struct component [`Username`].

use bevy::ecs::component::Component;

/// The player's username, as a `String`.
/// This is the human-readable name they have chosen to go by, e.g. `meloncrafter` or `Notch`.
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
/// It is highly recommended to use the [`Uuid`](`crate::Uuid`) type to identify a player,
/// instead of their username.
#[derive(Component)]
pub struct Username(

    /// The player's username, as a `String`. This is the human-readable name they have chosen to go
    /// by.
    /// See [`Username`] for more info.
    ///
    /// ## Protocol representation
    /// Serialized and deserialized as a normal string.
    pub String
);
