//! Module for struct resource [`OnlinePlayers`].

use bevy::ecs::resource::Resource;
use core::ops::{Deref, DerefMut};

/// The server's *current count* of the *number of players* that are online on the server at the
/// moment.
///
/// You may also want to reference this resource whenever a client sends a status request
/// (server list ping) , as the Minecraft server list shows the number of players currently online.
/// well as the maximum number of players that can join the server.
///
/// This is a global resource; you can add it with Bevy to change the number of displayed online
/// players.
/// You should probably insert this resource initially with a count of zero, then access and
/// increment it whenever a player joins/leaves the game.
///
/// ## Future breaking changes
/// In the future, this struct will likely also store the *list of player entities* that are online
/// on the server, instead of the number of online players.
#[derive(Resource)]
pub struct OnlinePlayers(

    /// The value of the current number of online players.
    /// See [`OnlinePlayers`] for more information.
    pub u32,
);

impl Deref for OnlinePlayers {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for OnlinePlayers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl Default for OnlinePlayers {
    fn default() -> Self {
        return Self(0);
    }
}
