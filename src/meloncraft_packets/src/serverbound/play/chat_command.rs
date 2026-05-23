use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolBuffer as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundChatCommand {
    pub client: Entity,
    pub command: String,
}

impl ServerboundPacket for ServerboundChatCommand {
    fn id() -> i32 {
        return 0x06
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;

        let command = incoming.data.net_deserialize().ok()?;

        return Some(Self {
            client,
            command,
        })
    }
}
