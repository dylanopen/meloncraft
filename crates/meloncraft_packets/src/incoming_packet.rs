use bevy::prelude::{Entity, Query};
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use std::fmt::Display;

pub enum IncomingPacketParseError {
    ClientNonExistent {
        packet_client: Entity,
    },
    UnmatchedState {
        packet_state: ConnectionState,
        required_state: ConnectionState,
    },
    UnmatchedId {
        required_id: i32,
        packet_id: i32,
    },
}

impl Display for IncomingPacketParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncomingPacketParseError::ClientNonExistent {
                packet_client: client,
            } => f.write_fmt(format_args!("Non-existent client: {client}",)),
            IncomingPacketParseError::UnmatchedState {
                packet_state,
                required_state,
            } => f.write_fmt(format_args!(
                "Unmatched state: packet={packet_state} -> required={required_state}",
            )),
            IncomingPacketParseError::UnmatchedId {
                packet_id,
                required_id,
            } => f.write_fmt(format_args!(
                "Unmatched id: packet={:?} -> required={:?}",
                packet_id, required_id
            )),
        }
    }
}

pub trait IncomingPacket: Sized {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self>;

    fn validate(
        incoming: &IncomingNetworkPacket,
        client_connections: &Query<&ClientConnection>,
    ) -> Result<(), IncomingPacketParseError> {
        let Ok(client_connection) = client_connections.get(incoming.client) else {
            return Err(IncomingPacketParseError::ClientNonExistent {
                packet_client: incoming.client,
            });
        };
        if client_connection.state != Self::state() {
            return Err(IncomingPacketParseError::UnmatchedState {
                packet_state: client_connection.state,
                required_state: Self::state(),
            });
        }
        if incoming.id != Self::id() {
            return Err(IncomingPacketParseError::UnmatchedId {
                packet_id: incoming.id,
                required_id: Self::id(),
            });
        }
        Ok(())
    }

    fn from_packet(
        incoming: &IncomingNetworkPacket,
        client_connections: &Query<&ClientConnection>,
    ) -> Option<Self> {
        Self::validate(incoming, client_connections).ok()?;
        Self::parse(incoming)
    }
}
