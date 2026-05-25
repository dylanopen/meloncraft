use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_player::Difficulty;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundChangeDifficulty {
    pub client: Entity,
    pub new_difficulty: Difficulty,
}

impl ServerboundPacket for ServerboundChangeDifficulty {
    fn id() -> i32 {
        return 0x03
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;

        let new_difficulty = u8::net_deserialize(&mut incoming.data).ok()?.try_into().ok()?;

        return Some(Self {
            client,
            new_difficulty,
        })
    }
}
