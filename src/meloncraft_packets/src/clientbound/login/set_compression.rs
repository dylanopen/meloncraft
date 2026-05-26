use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ClientboundSetCompression {
    pub client: Entity,
    pub threshold: i32,
}

impl ClientboundPacket for ClientboundSetCompression {
    fn id() -> i32 {
        return 0x03
    }

    fn state() -> ConnectionState {
        return ConnectionState::Login
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.threshold.net_serialize());
    }
}
