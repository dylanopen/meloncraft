use core::fmt;
use core::fmt::Display;

/// The connection state of a client, each client can be in one of five states:
/// - [`ConnectionState::Handshaking`]
/// - [`ConnectionState::Status`]
/// - [`ConnectionState::Login`]
/// - [`ConnectionState::Configuration`]
/// - [`ConnectionState::Play`]
///
/// The connection state is used by the network crate to determine how to parse serverbound packets
/// from the client.
/// If the client's connection state is not updated correctly, then packets may be parsed
/// incorrectly, because of the Minecraft protocol's design.
///
/// ## Connection state in the protocol
/// - Every packet in the Minecraft protocol has a unique ID.
/// - ...However that ID is only unique within a specific *connection state*.
/// - For example, multiple packets have the ID `0x00`, but they are differentiated by the
///   connection state. The `Handshake` packet has the ID `0x00` in the `Handshaking` state, while the
///   `LoginStart` packet has the ID `0x00` in the `Login` state.
/// - That means it's really important to update the connection state of a client (see
///   [`ClientConnection::state`]) whenever they change connection state, for example, when they
///   finish [`ConnectionState::Login`] and move on to [`ConnectionState::Configuration`].
/// - If not updated correctly, then Meloncraft may try to parse a packet with the wrong connection
///   state, which will cause the packet to be parsed incorrectly or even cause the server to crash
///   or the connection to be terminated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

impl Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            ConnectionState::Handshaking => write!(f, "Handshaking"),
            ConnectionState::Status => write!(f, "Status"),
            ConnectionState::Login => write!(f, "Login"),
            ConnectionState::Configuration => write!(f, "Configuration"),
            ConnectionState::Play => write!(f, "Play"),
        };
    }
}
