use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;

/// Packet telling the client to remove all existing title messages from the screen.
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Clear_Titles>.
#[derive(Message, Debug, Clone)]
pub struct ClientboundClearTitles {
    pub client: Entity,

    pub reset: bool,
}

impl ClientboundPacket for ClientboundClearTitles {
    fn id() -> i32 {
        return 0x0E;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.reset.net_serialize());
    }
}
