use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use meloncraft_protocol_types::deserialize;
use crate::IncomingPacket;

#[derive(Message, Debug)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ConnectionState,
}

impl IncomingPacket for Handshake {
    fn id() -> i32 { 0x00 }
    fn state() -> ConnectionState { ConnectionState::Handshaking }
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        let mut incoming = incoming.clone();
        let protocol_version = deserialize::varint(&mut incoming.data).ok()?;
        let server_address = deserialize::string(&mut incoming.data).ok()?;
        let server_port = deserialize::unsigned_short(&mut incoming.data).ok()?;
        let next_state = ConnectionState::Handshaking;

        Some(Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
