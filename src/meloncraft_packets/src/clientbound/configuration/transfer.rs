use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ClientboundTransfer {
    pub client: Entity,
    pub hostname: String,
    pub port: u16,
}

impl ClientboundPacket for ClientboundTransfer {
    fn id() -> i32 {
        return 0x0B;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.hostname.net_serialize());
        data.extend(VarInt(self.port.into()).net_serialize());
    }
}
