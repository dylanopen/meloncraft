use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::GameMode;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundChangeGamemode {
    pub client: Entity,
    pub new_gamemode: GameMode,
}

impl ServerboundPacket for ServerboundChangeGamemode {
    fn id() -> i32 {
        return 0x04
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;

        let new_gamemode = u8::net_deserialize(&mut incoming.data).ok()?.try_into().ok()?;

        return Some(Self {
            client,
            new_gamemode,
        })
    }
}
