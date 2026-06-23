use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;
use meloncraft_text::NbtText;

/// Packet to display an actionbar message (appears above the client's hotbar).
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Action_Bar_Text>.
///
/// Functionally identical to `ClientboundSetTitleText`.
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetActionbarText {
    pub client: Entity,

    /// The action bar, as [`NbtText`], to display to the client.
    pub title: NbtText,
}

impl ClientboundPacket for ClientboundSetActionbarText {
    fn id() -> i32 {
        return 0x55;
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
