use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct AcknowledgeFinishConfiguration {
    pub client: Entity,
}

impl ServerboundPacket for AcknowledgeFinishConfiguration {
    fn id() -> i32 {
        0x03
    }
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }
    fn deserialize(incoming: &ServerboundNetworkPacket) -> Option<Self> {
        let client = incoming.client;
        Some(AcknowledgeFinishConfiguration { client })
    }
}
