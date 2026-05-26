use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::{Byte, PrefixedArray, ProtocolType as _};

#[derive(Message, Debug, Clone)]
pub struct ClientboundEncryptionRequest {
    pub client: Entity,
    pub server_id: String,
    pub public_key: Vec<Byte>,
    pub verify_token: Vec<Byte>,
    pub should_authenticate: bool,
}

impl ClientboundPacket for ClientboundEncryptionRequest {
    fn id() -> i32 {
        return 0x01
    }
    fn state() -> ConnectionState {
        return ConnectionState::Login
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.append(&mut self.server_id.net_serialize());
        data.append(&mut PrefixedArray::from(self.public_key.clone()).net_serialize());
        data.append(&mut PrefixedArray::from(self.verify_token.clone()).net_serialize());
        data.append(&mut self.should_authenticate.net_serialize());
    }
}
