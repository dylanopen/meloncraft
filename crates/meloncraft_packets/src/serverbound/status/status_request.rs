use crate::ServerboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct StatusRequest {
    pub client: Entity,
}

impl ServerboundPacket for StatusRequest {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Status
    }
    fn deserialize(incoming: &ServerboundNetworkPacket) -> Option<Self> {
        Some(StatusRequest {
            client: incoming.client,
        })
    }
}
