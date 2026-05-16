use core::fmt;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum ServerboundPacketParseError {
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

impl Display for ServerboundPacketParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerboundPacketParseError::ClientNonExistent {
                packet_client: client,
            } => f.write_fmt(format_args!("Non-existent client: {client}")),
            ServerboundPacketParseError::UnmatchedState {
                packet_state,
                required_state,
            } => f.write_fmt(format_args!(
                "Unmatched state: packet={packet_state} -> required={required_state}",
            )),
            ServerboundPacketParseError::UnmatchedId {
                packet_id,
                required_id,
            } => f.write_fmt(format_args!(
                "Unmatched id: packet={packet_id:?} -> required={required_id:?}",
            )),
        }
    }
}

pub trait ServerboundPacket: Sized + Message + Debug + Clone {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self>;

    fn validate(incoming: &ServerboundNetworkPacket) -> Result<(), ServerboundPacketParseError> {
        if incoming.state != Self::state() {
            return Err(ServerboundPacketParseError::UnmatchedState {
                packet_state: incoming.state,
                required_state: Self::state(),
            });
        }
        if incoming.id != Self::id() {
            return Err(ServerboundPacketParseError::UnmatchedId {
                packet_id: incoming.id,
                required_id: Self::id(),
            });
        }
        Ok(())
    }

    #[must_use]
    fn from_packet(incoming: &ServerboundNetworkPacket) -> Option<Self> {
        Self::validate(incoming).ok()?;
        Some(Self::deserialize(incoming).unwrap())
    }
}
