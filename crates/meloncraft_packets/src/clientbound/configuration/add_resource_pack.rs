use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::ProtocolType;
use meloncraft_text::NbtText;

#[derive(Message, Debug, Clone)]
pub struct AddResourcePack {
    pub client: Entity,
    pub resource_pack_uuid: Uuid,
    pub url: String,
    pub sha1_hash: String,
    pub force_accept: bool,
    pub prompt_message: Option<NbtText>,
}

impl ClientboundPacket for AddResourcePack {
    fn id() -> i32 {
        0x09
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = self.resource_pack_uuid.net_serialize();
        data.extend(self.url.net_serialize());
        data.extend(self.sha1_hash.net_serialize());
        data.extend(self.force_accept.net_serialize());
        data.extend(self.prompt_message.net_serialize());
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
