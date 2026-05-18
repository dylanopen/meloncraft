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

    /// The client intends to query the server's status.
    ///
    /// The next packet will be sent in the `Status` connection state, and the clien'ts connection
    /// state should be updated to suit this (or the packet will be parsed incorrectly).
    /// **The only state that should follow this is the `Status` connection state.**
    ///
    /// This intent is sent when the client is pinging the server in the server list, to get the
    /// server's MOTD, player count, ping, etc.
    Status,

    /// The client intends to log in to the server.
    ///
    /// You should update the next state to `ConnectionState::Login` when receiving this
    /// intention, so that the next packet is parsed as a `Login` packet.
    /// You should do this even if you don't want them to log in: you should probably disconnect them
    /// gracefully later on in the `Login` connection state instead of killing the connection now.
    ///
    /// This intention is sent when the client clicks on our server from the server list, and is
    /// trying to log in to the server directly. If they are transferred to our server from another
    /// server, then they will send [`IntentionType::Transfer`] instead.
    Login,

    /// The client intends to join our server because they have been sent here *from* another server.
    ///
    /// You may want to handle this differently if you don't accept transfers. **However** you
    /// should still update the client's connection state to `ConnectionState::Login` when receiving
    /// this intention, so that the next packet is parsed as a `Login` packet.
    ///
    /// If you want to do something special for transferred clients, consider adding a component to
    /// the client's `Entity` when receiving this intention, and then checking for this entity
    /// during the `Login`, `Configuration` or `Play` connection states (depending on how you want
    /// to handle it). Consider disconnecting transferred clients during the `Login` state if you
    /// don't want to accept transfers.
    ///
    /// This intention is sent because a different server sent a transfer packet to the client, and
    /// the target server was ours.
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
