use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundKeepAlive {
    pub client: Entity,
}

impl ClientboundPacket for ClientboundKeepAlive {
    fn id() -> i32 {
        return 0x04
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {}
}
