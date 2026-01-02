use crate::IncomingPacket;
use bevy::prelude::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use meloncraft_protocol_types::deserialize;

#[derive(Message, Debug)]
pub struct Ping {
    pub timestamp: i64,
}

impl IncomingPacket for Ping {
    fn id() -> i32 {
        0x01
    }
    fn state() -> ConnectionState {
        ConnectionState::Status
    }
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        let mut incoming = incoming.clone();
        let timestamp = deserialize::long(&mut incoming.data).unwrap();
        Some(Self { timestamp })
    }
}
