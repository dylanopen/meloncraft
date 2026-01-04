use crate::ServerboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType, VarInt};

#[derive(Message, Debug, Clone)]
pub struct Intention {
    pub client: Entity,
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ConnectionState,
}

impl ServerboundPacket for Intention {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Handshaking
    }
    fn deserialize(incoming: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = incoming.clone();
        let protocol_version = VarInt::net_deserialize(&mut incoming.data).unwrap().0;
        let server_address = String::net_deserialize(&mut incoming.data).unwrap();
        let server_port = u16::net_deserialize(&mut incoming.data).unwrap();
        let next_state = match VarInt::net_deserialize(&mut incoming.data).unwrap().0 {
            1 => ConnectionState::Status,
            2 | 3 => ConnectionState::Login,
            _ => return None, // TODO: log this
        };

        Some(Intention {
            client: incoming.client,
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
