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

    /// Handshaking is always the first [`ConnectionState`] that a client is in when they connect to
    /// the server.
    ///
    /// There is only one packet in the modern protocol that is sent in the `Handshaking` state, the
    /// `Intention` packet, which is sent by the **client** (it is a serverbound packet) to indicate
    /// the client's intention to either query the server's status, log in to the server, or
    /// transfer to another server.
    ///
    /// The packet in this state's primary purpose is to *change* the client's [`ConnectionState`]
    /// to either [`ConnectionState::Status`], [`ConnectionState::Login`].
    ///
    /// The client actually sends its second packet (in a different state) directly after this
    /// handshaking packet. It is important that you update the state on the first time the
    /// handshake packet is received, so that the second packet is parsed with the correct
    /// connection state.\
    /// To avoid parsing the second packet as a `Handshaking` packet, the default network crate will
    /// wait `60ms` after receiving the handshake packet before it starts parsing the next packet(s)
    /// from the client, to give your systems time to update the client's connection state.
    Handshaking,

    /// The `Status` [`ConnectionState`] is used when a client is querying the server's status,
    /// when the client is pinging the server in the server list.
    ///
    /// ## Predecessor states
    /// - **Status**: The client can only go to the `Status` state from the `Handshaking` state, by
    ///   sending the `Intention` packet with the `next_state` field set to `IntentionType::Status`.
    /// - The client cannot be transferred to the `Status` state from any other state.
    /// 
    /// ## Successive states
    /// - This state is terminal: a client cannot go from the `Status` state to any other
    ///   [`ConnectionState`], and in the modern protocol (pre-1.7 worked a bit differently), the
    ///   connection is terminated after the client finishes querying the server's status.
    /// - No successive states.
    ///
    Status,

    /// The `Login` [`ConnectionState`] is used when a client is logging in to the server, but
    /// before they send+receive the configuration needed to join the world.
    ///
    /// See the documentation for the `meloncraft_packets` crate for the packets sent in this state.
    ///
    /// ## Purpose
    /// The login state does a few main things:
    /// - Authenticates the client with Mojang's authentication servers, if the server is in
    ///   online-mode
    /// - Configures **encryption** to use when communicating
    /// - Configures **compression** to send packets with and when to use it (what size the packets
    ///   need to be before they start being compressed)
    /// - Configures plugin channels that the client and server can use to send custom payloads to
    ///   each other. This is mainly used for **mods**.
    /// - Set any cookies that the server wants to.
    /// - Collect the player's username and UUID.
    /// - Various other things that the server may need to get info about the client.
    ///
    /// ## Predecessor states
    /// - **Handshaking**: The client can only go to the `Login` state from the
    ///   [`ConnectionState::Handshaking`] state, by sending the `Intention` packet with the
    ///   `next_state` field set to `IntentionType::Login` **or** `IntentionType::Transfer`.
    ///
    /// ## Successive states
    /// - **Configuration**: The client can only go to the `Configuration` state from the `Login`
    ///   state, after the client has sent the `LoginStart` packet and received the `LoginSuccess` packet.
    Login,

    /// During the `Configuration` [`ConnectionState`], the client and server tell each other about
    /// the settings they want to use, share registry and datapack information, and do various other
    /// things to configure the client's connection to the server before the client will actually be
    /// able to load into the world.
    ///
    /// See the documentation for the `meloncraft_packets` crate for the packets sent in this state.
    ///
    /// ## Purpose
    /// - Server and client share settings and information about the connection, such as view
    ///   view distance, chat settings, and other **client information**.
    /// - The server and client exchange info on which datapacks they both know about. The result of
    ///   this depends on what the server should then send.
    /// - The server sends any **registries** it wants to use. Either just the IDs if the client
    ///   knows the pack, or the full data if it doesn't. Currently, only sending the registry
    ///   structure is supported (not the full data). This may change.
    /// - Any other information the server or client wants to send.
    ///
    /// ## Predecessor states
    /// **Login**: the client needs to go through the login state *before* it enters the
    /// configuration state. It enters [`ConnectionState::Configuration`] after the client has sent
    /// the `LoginAcknowledged` packet.
    ///
    /// ## Successive states
    /// **Play**: when configuration has finished, the server should set the client's connection
    /// state to [`ConnectionState::Play`].
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
