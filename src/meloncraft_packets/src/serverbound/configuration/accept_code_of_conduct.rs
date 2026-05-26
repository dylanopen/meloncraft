use crate::ServerboundPacket;
use crate::network_messages::ServerboundNetworkPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;

#[derive(Message, Debug, Clone)]
pub struct ServerboundAcceptCodeOfConduct {
    pub client: Entity,
}

impl ServerboundPacket for ServerboundAcceptCodeOfConduct {
    fn id() -> i32 {
        return 0x09;
    }
    fn state() -> ConnectionState {
        return ConnectionState::Configuration;
    }

    fn deserialize(packet: ServerboundNetworkPacket) -> Option<Self> {
        let client = packet.client;

        return Some(Self { client });
    }
}
