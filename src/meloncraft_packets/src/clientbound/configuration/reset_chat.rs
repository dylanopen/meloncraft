use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;

#[derive(Message, Debug, Clone)]
pub struct ClientboundResetChat {
    pub client: Entity,
}

impl ClientboundPacket for ClientboundResetChat {
    fn id() -> i32 {
        return 0x06;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, _data: &mut Vec<u8>) {}
}
