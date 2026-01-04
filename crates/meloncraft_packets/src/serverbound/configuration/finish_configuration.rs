use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct FinishConfiguration {
    pub client: Entity,
}

impl ServerboundPacket for FinishConfiguration {
    fn id() -> i32 {
        0x03
    }
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let client = packet.client;

        Some(Self { client })
    }
}
