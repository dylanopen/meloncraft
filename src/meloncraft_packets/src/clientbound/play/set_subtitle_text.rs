use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;
use meloncraft_text::NbtText;

/// Packet to display a subtitle message (message in the middle of the screen, just below
/// the title) to a client.
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Subtitle_Text>.
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetSubtitleText {
    pub client: Entity,

    /// The title, as [`NbtText`], to display to the client.
    pub title: NbtText,
}

impl ClientboundPacket for ClientboundSetSubtitleText {
    fn id() -> i32 {
        return 0x6E;
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
