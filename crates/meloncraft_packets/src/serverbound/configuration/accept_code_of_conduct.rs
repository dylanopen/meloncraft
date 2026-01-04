use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct AcceptCodeOfConduct {
    pub client: Entity,
}

impl ServerboundPacket for AcceptCodeOfConduct {
    fn id() -> i32 {
        0x09
    }
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let client = packet.client;

        Some(Self { client })
    }
}
