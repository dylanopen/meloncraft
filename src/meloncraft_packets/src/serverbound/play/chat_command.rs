use crate::ServerboundPacket;
use crate::network_messages::ServerboundNetworkPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolBuffer as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundChatCommand {
    pub client: Entity,
    pub command: String,
}

impl ServerboundPacket for ServerboundChatCommand {
    fn id() -> i32 {
        return 0x06;
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }
    fn deserialize(mut packet: ServerboundNetworkPacket) -> Option<Self> {
        let client = packet.client;

        let command = packet.data.net_deserialize().ok()?;

        return Some(Self { client, command });
    }
}
