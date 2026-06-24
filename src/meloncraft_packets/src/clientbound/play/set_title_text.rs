use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;
use meloncraft_text::NbtText;

/// Packet to Display a title message (big message in the middle of the screen) to a client.
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Title_Text>.
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetTitleText {
    pub client: Entity,

    /// The title, as [`NbtText`], to display to the client.
    pub title: NbtText,
}

impl ClientboundPacket for ClientboundSetTitleText {
    fn id() -> i32 {
        return 0x70;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.title.net_serialize());
    }
}
