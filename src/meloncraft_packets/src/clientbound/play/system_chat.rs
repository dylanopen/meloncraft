use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;
use meloncraft_text::NbtText;

/// Send a system chat message with formatted text to a client.
///
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#System_Chat_Message>.
#[derive(Message, Debug, Clone)]
pub struct ClientboundSystemChat {
    pub client: Entity,

    /// A text component representing the contents of the chat message to send to the client.
    pub message: NbtText,

    /// Whether the message is an actionbar or chat message.
    /// - `true`: action bar
    /// - `false`: chat message
    pub overlay: bool,
}

impl ClientboundPacket for ClientboundSystemChat {
    fn id() -> i32 {
        return 0x54;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.message.net_serialize());
    }
}
