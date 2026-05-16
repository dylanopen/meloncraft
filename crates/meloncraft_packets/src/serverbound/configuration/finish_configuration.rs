use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct ServerboundFinishConfiguration {
    pub client: Entity,
}

impl ServerboundPacket for ServerboundFinishConfiguration {
    fn id() -> i32 {
        return 0x03
    }
    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let client = packet.client;

        return Some(Self { client })
    }
}
