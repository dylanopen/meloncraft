use crate::ServerboundPacket;
use crate::network_messages::ServerboundNetworkPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;

#[derive(Message, Debug, Clone)]
pub struct ServerboundStatusRequest {
    pub client: Entity,
}

impl ServerboundPacket for ServerboundStatusRequest {
    fn id() -> i32 {
        return 0x00;
    }
    fn state() -> ConnectionState {
        return ConnectionState::Status;
    }
    fn deserialize(packet: ServerboundNetworkPacket) -> Option<Self> {
        return Some(ServerboundStatusRequest {
            client: packet.client,
        });
    }
}
