//! Module for the [`ClientConnection`] struct, which is used to store data about a client's
//! connection to the server.
//!
//! Used primarily by the `meloncraft_network` crate to identify clients by their IP address and
//! connection state, and to send and receive packets to and from the client.
//! Can also be used by any other plugins which need to:
//! - Get the IP address of a client.
//! - Get **or set** the [`ConnectionState`] of a client.
//!
//! Primary docs: [`ClientConnection`].

use crate::connection_state::ConnectionState;
use bevy::ecs::component::Component;
use core::net::SocketAddr;
use std::net::TcpStream;

/// Container for data about a client, used in the network code.
/// Used primarily by the `meloncraft_network` crate to identify a client
/// by their IP (sochet address).
///
/// ## Stored data
/// - The [`TcpStream`] for the client's network connection.
/// - The client's IP address, [`SocketAddr`]. This is used to identify them when they send serverbound
///   packets, as well as to send clientbound packets back to them.
/// - The client's connection state, [`ConnectionState`], so that packets can be parsed differently
///   depending on what state (handshaking, status, login, configuration or play) the client is in.
///
/// ## ECS
/// - [`ClientConnection`] is a Bevy component. It should be attached to the bevy `Entity` that
///   stores all the data about the client and player.
/// - You can query for [`ClientConnection`] in systems to get the client's IP address and
///   connection state.
/// - You can modify the connection state using the [`ClientConnection::state`] field. You should not
///   modify the other fields, as they are used by the network crate to send and receive
///   packets to and from the client. Modifying them may break the netcode.
#[derive(Component, Debug)]
pub struct ClientConnection {
    /// The client's IP address, as a [`SocketAddr`].
    ///
    /// ## Modification
    /// - Modifying this field **will** mean that serverbound packets will be missed from that client.
    /// - While the network crate should still send clientbound packets to the old address (as the
    ///   [`TcpStream`] is still the same), it may cause issues and modification is still strongly
    ///   discouraged.
    ///
    /// ## Identification
    /// - This is the main way that the client is identified in netcode.
    /// - When a packet is received from a client, the network crate will check the source IP
    ///   address of the packet and compare it to the [`address`] field of all [`ClientConnection`]s
    ///   to find out which client sent the packet, and which client to add to the serverbound
    ///   packet's data.
    pub address: SocketAddr,

    /// The TCP connection with the client, as a [`TcpStream`].
    ///
    /// ## Modification
    /// - You should never use or modify this field manually, as the `meloncraft_packets` crate
    ///   provides type-safe functions to send packets to the client.
    /// - It is used by the network crate to send and receive packets to and from the client.
    ///   Modifying it may break the netcode.
    /// - If you would like to create a **custom** serverbound or clientbound packet, please
    ///   see the `meloncraft_packets` crate's source for each packet. Do not interact with the
    ///   [`TcpStream`] directly.
    pub tcp_stream: TcpStream,

    /// The player's current [`ConnectionState`]. This is currently used to determine how to parse
    /// serverbound packets. See the [`ConnectionState`] documentation for more information.
    ///
    /// The connectionState is specific to each client.
    ///
    /// ## Modification
    /// You should modify this field whenever a client changes connection state, for example, when
    /// they finish the [`ConnectionState::Handshaking`] state and move to the
    /// [`ConnectionState::Login`] state.
    ///
    /// There are usually already plugins for Meloncraft which handle this state for you: for
    /// example, the `meloncraft_packet_forwarding` plugin has a system that listens for the
    /// `LoginHandshaken` message sent by the `meloncraft_handshaking` crate, and changes the
    /// client's connection state to [`ConnectionState::Login`] or [`ConnectionState::Status`]
    /// depending on the client's intention.
    pub state: ConnectionState,

    /// The number of packets the client has sent to the server, which the server has deserialized.
    ///
    /// This increases by one every time the server reads a packet from the client.
    ///
    /// ## Usage
    /// - Currently, this is used to delay packet processing, in the case of *handshaking*. The
    ///   client sends both the handshake and status/login packet in one stream, and the non-network
    ///   systems need time to react and change the `ConnectionState`.
    /// - See the `read_streams` system in the network crate for more details.
    pub serverbound_packets_processed: usize,
}

impl Clone for ClientConnection {
    fn clone(&self) -> Self {
        return ClientConnection {
            tcp_stream: self.tcp_stream.try_clone().unwrap(),
            state: self.state,
            address: self.address,
            serverbound_packets_processed: self.serverbound_packets_processed,
        };
    }
}
