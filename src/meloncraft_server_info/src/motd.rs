//! Module for struct resource [`Motd`].

use bevy::ecs::resource::Resource;
use core::ops::{Deref, DerefMut};

/// The server's *message of the day* (`MOTD`), as a `String`.
///
/// This can be thought of as the server *description*: it's the message that shows on the
/// multiplayer server list screen, and is sent to clients in the `Status` packet by the
/// `meloncraft_server_list` logic crate.
///
/// This is a global resource; you can add it with Bevy to change the MOTD.
/// You should insert this resource to define the MOTD of your server, and you can change it at
/// runtime to change the MOTD of your server on the fly.
#[derive(Resource, Debug, Clone)]
pub struct Motd(

    /// The string representing the value of the server's MOTD.
    /// This is the message that shows on the multiplayer server list screen for your server, see
    /// [`Motd`] for more info.
    pub String,
);

// TODO: make `Motd` hold an `NbtText` component, not a raw, unformatted string.

impl Deref for Motd {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DerefMut for Motd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
