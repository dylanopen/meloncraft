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
    /// ## Identification
    /// - This is the main way that the client is identified in netcode.
    /// - When a packet is received from a client, the network crate will check the source IP
    ///   address of the packet and compare it to the [`address`] field of all [`ClientConnection`]s
    ///   to find out which client sent the packet, and which client to add to the serverbound
    ///   packet's data.
    pub address: SocketAddr,

    /// The TCP connection with the client, as a [`TcpStream`].
    /// ## Modification
    /// - You should never use or modify this field manually, as the `meloncraft_packets` crate
    ///   provides type-safe functions to send packets to the client.
    /// - It is used by the network crate to send and receive packets to and from the client.
    ///   Modifying it may break the netcode.
    /// - If you would like to create a **custom** serverbound or clientbound packet, please
    ///   see the `meloncraft_packets` crate's source for each packet. Do not interact with the
    ///   [`TcpStream`] directly.
    pub tcp_stream: TcpStream,

    pub state: ConnectionState,
}

impl Clone for ClientConnection {
    fn clone(&self) -> Self {
        return ClientConnection {
            tcp_stream: self.tcp_stream.try_clone().unwrap(),
            state: self.state,
            address: self.address,
        };
    }
}
