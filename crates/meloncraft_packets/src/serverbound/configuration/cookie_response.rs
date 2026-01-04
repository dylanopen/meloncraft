use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::{Byte, PrefixedArray, ProtocolBuffer, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct CookieResponse {
    pub client: Entity,
    pub key: Identifier,
    pub value: Vec<Byte>,
}

impl ServerboundPacket for CookieResponse {
    fn id() -> i32 {
        0x01
    }
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let key = data.net_deserialize().unwrap();
        let value = PrefixedArray::<Byte>::net_deserialize(&mut data).unwrap().0;

        Some(Self { client, key, value })
    }
}
