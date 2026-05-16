use crate::ServerboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct ServerboundStatusRequest {
    pub client: Entity,
}

impl ServerboundPacket for ServerboundStatusRequest {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Status
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        Some(ServerboundStatusRequest {
            client: packet.client,
        })
    }
}
