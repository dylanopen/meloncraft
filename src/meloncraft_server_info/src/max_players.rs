//! Module for struct resource [`MaxPlayers`].

use bevy::ecs::resource::Resource;
use core::ops::{Deref, DerefMut};

/// The server's *limit* to the *number of players* that can join the server, as a `u32`.
///
/// This should be referenced whenever checking whether a player can join, as well as in response to
/// a status request, as the Minecraft server list shows the number of players currently online, as
/// well as the maximum number of players that can join the server.
///
/// This is a global resource; you can add it with Bevy to change the max number of players.
/// You should insert this resource to define the player limit of your server, and you can change it at
/// runtime to change the max player count of your server while it's running.
#[derive(Resource)]
pub struct MaxPlayers(
    /// The `u32` value representing the maximum number of players that can join the server.
    /// See [`MaxPlayers`] for more information.
    pub u32,
);

impl Deref for MaxPlayers {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for MaxPlayers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Default for MaxPlayers {
    fn default() -> Self {
        return Self(20);
    }
}
