use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ServerboundAcknowledgeChat {
    pub client: Entity,
    pub message_count: i32,
}

impl ServerboundPacket for ServerboundAcknowledgeChat {
    fn id() -> i32 {
        return 0x05
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(mut packet: ServerboundNetworkPacket) -> Option<Self> {
        let client = packet.client;

        let message_count = VarInt::net_deserialize(&mut packet.data).ok()?.0;

        return Some(Self {
            client,
            message_count,
        })
    }
}
