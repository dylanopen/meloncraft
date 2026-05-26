use crate::ServerboundPacket;
use crate::network_messages::ServerboundNetworkPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_protocol_types::{Byte, PrefixedArray, ProtocolBuffer as _, ProtocolType as _};

#[derive(Message, Debug, Clone)]
pub struct ServerboundCookieResponse {
    pub client: Entity,
    pub key: Identifier,
    pub value: Vec<Byte>,
}

impl ServerboundPacket for ServerboundCookieResponse {
    fn id() -> i32 {
        return 0x01;
    }
    fn state() -> ConnectionState {
        return ConnectionState::Configuration;
    }

    fn deserialize(packet: ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let key = data.net_deserialize().unwrap();
        let value = PrefixedArray::<Byte>::net_deserialize(&mut data).unwrap().0;

        return Some(Self { client, key, value });
    }
}
