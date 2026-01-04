use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_nbt::NbtTag;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct ShowDialog {
    pub client: Entity,
    pub dialog: NbtTag,
}

impl ClientboundPacket for ShowDialog {
    fn id() -> i32 {
        0x12
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let data = self.dialog.net_serialize();
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
