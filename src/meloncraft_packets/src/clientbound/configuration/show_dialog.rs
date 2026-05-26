use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_nbt::NbtTag;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ClientboundShowDialog {
    pub client: Entity,
    pub dialog: NbtTag,
}

impl ClientboundPacket for ClientboundShowDialog {
    fn id() -> i32 {
        return 0x12
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.dialog.net_serialize());
    }
}
