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
    
    pub address: SocketAddr,

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
