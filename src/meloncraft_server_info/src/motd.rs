//! Module for struct resource [`Motd`].

use bevy::ecs::resource::Resource;
use core::ops::{Deref, DerefMut};

/// The server's *message of the day* (`MOTD`), as a `String`.
///
/// This can be thought of as the server *description*: it's the message that shows on the
/// multiplayer server list screen, and is sent to clients in the `Status` packet by the
/// `meloncraft_server_list` logic crate.
///
/// You should insert this resource to define the MOTD of your server, and you can change it at
/// runtime to change the MOTD of your server on the fly.
#[derive(Resource)]
pub struct Motd(pub String);

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
