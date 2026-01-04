use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct SetFeatureFlags {
    pub client: Entity,
    pub feature_flags: Vec<Identifier>,
}

impl ClientboundPacket for SetFeatureFlags {
    fn id() -> i32 {
        0x0C
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let data = PrefixedArray(self.feature_flags.clone()).net_serialize();
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
