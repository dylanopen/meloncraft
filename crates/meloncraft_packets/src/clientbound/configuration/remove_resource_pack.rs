use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Clone, Debug)]
pub struct RemoveResourcePack {
    pub client: Entity,
    pub resource_pack_uuid: Uuid,
}

impl ClientboundPacket for RemoveResourcePack {
    fn id() -> i32 {
        0x08
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.resource_pack_uuid.net_serialize(),
        })
    }
}
