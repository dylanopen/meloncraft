//! Module for [`MeloncraftHandshakingPlugin`] and submodules related to receiving and responding
//! to handshake requests from clients.

pub mod handshaken;

use bevy::app::Plugin;
use bevy::app::App;

use self::handshaken::LoginHandshaken;
use self::handshaken::StatusHandshaken;
use self::handshaken::TransferHandshaken;

pub struct MeloncraftHandshakingPlugin;

/// Plugin to register Bevy messages for the `handshaking` crate, messages related to receiving and
/// responding to handshake requests from clients.
///
/// ## Registered messages
/// - **[`StatusHandshaken`]**: a client has sent a handshake to the server, with their next
///   *intention* being `Status`. They are pinging the server information from the multiplayer menu.
/// - **[`LoginHandshaken`]**: a client has sent a handshake to the server, with their next
///   *intention* being `Login`. They joined from the multiplayer menu.
/// - **[`TransferHandshaken`]**: a client has sent a handshake to the server, with their next
///   *intention* being `Login`, **but** they were transferred to this server from a different
///   server.
impl Plugin for MeloncraftHandshakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StatusHandshaken>();
        app.add_message::<LoginHandshaken>();
        app.add_message::<TransferHandshaken>();
    }
}
