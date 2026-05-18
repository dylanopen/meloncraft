use core::fmt;

/// The intention type of a handshake packet.
/// Indicates whether the client is trying to:
/// - Query the server's status, done when pinging the server in the server list -
///   [`IntentionType::Status`].
/// - Log in to the server, which happens when they click on our server from the server list -
///   [`IntentionType::Login`].
/// - Join our server because they have been sent here *from* another server, which happens when
///   they are transferred to our server from another server - [`IntentionType::Transfer`] (*we may
///   want to handle this differently if we don't accept transfers*).
///
/// ## Connection state
/// - The [`IntentionType`] is slightly different from the `ConnectionState` of a client.
/// - The `ConnectionState` is the client's current connection state, which is used to determine how
///   to parse any serverbound packets from the client.
/// - The [`IntentionType`] is the client's intention when they send the handshake packet, which
///   *may be* used to determine the *next* `ConnectionState` of the client, but is not usually
///   the same as the client's **current** `ConnectionState`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IntentionType {

    Status,

    Login,

    Transfer,
}

impl fmt::Display for IntentionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            IntentionType::Status => write!(f, "Status"),
            IntentionType::Login => write!(f, "Login"),
            IntentionType::Transfer => write!(f, "Transfer"),
        };
    }
}
