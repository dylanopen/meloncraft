use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ClientboundCookieRequest {
    pub client: Entity,
    pub key: Identifier,
}

impl ClientboundPacket for ClientboundCookieRequest {
    fn id() -> i32 {
        return 0x00;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.key.net_serialize());
    }
}
