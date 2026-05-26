use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType as _};

#[derive(Message, Debug, Clone)]
pub struct ClientboundSetFeatureFlags {
    pub client: Entity,
    pub feature_flags: Vec<Identifier>,
}

impl ClientboundPacket for ClientboundSetFeatureFlags {
    fn id() -> i32 {
        return 0x0C
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(PrefixedArray(self.feature_flags.clone()).net_serialize());
    }
}
