use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_protocol_types::{Byte, PrefixedArray, ProtocolType as _};

#[derive(Message, Debug, Clone)]
pub struct ClientboundStoreCookie {
    pub client: Entity,
    pub key: Identifier,
    pub value: Vec<Byte>,
}

impl ClientboundPacket for ClientboundStoreCookie {
    fn id() -> i32 {
        return 0x0A;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.key.net_serialize());
        data.extend(PrefixedArray(self.value.clone()).net_serialize());
    }
}
