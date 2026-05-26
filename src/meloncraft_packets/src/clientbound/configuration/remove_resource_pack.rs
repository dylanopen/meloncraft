use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Clone, Debug)]
pub struct ClientboundRemoveResourcePack {
    pub client: Entity,
    pub resource_pack_uuid: Uuid,
}

impl ClientboundPacket for ClientboundRemoveResourcePack {
    fn id() -> i32 {
        return 0x08
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.resource_pack_uuid.net_serialize());
    }
}
