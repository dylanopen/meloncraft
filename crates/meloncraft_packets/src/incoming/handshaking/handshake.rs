use crate::IncomingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use meloncraft_protocol_types::deserialize;

#[derive(Message, Debug)]
pub struct Intention {
    pub client: Entity,
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ConnectionState,
}

impl IncomingPacket for Intention {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Handshaking
    }
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        dbg!(incoming);
        let mut incoming = incoming.clone();
        let protocol_version = deserialize::varint(&mut incoming.data).unwrap();
        let server_address = deserialize::string(&mut incoming.data).unwrap();
        let server_port = deserialize::unsigned_short(&mut incoming.data).unwrap();
        let next_state = match deserialize::varint(&mut incoming.data).unwrap() {
            1 => ConnectionState::Status,
            2 | 3 => ConnectionState::Login,
            _ => return None, // TODO: log this
        };
        dbg!(&incoming.data);

        Some(Intention {
            client: incoming.client,
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
