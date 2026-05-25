use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_server_info::difficulty::Difficulty;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundChangeDifficulty {
    pub client: Entity,
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}

impl ClientboundPacket for ClientboundChangeDifficulty {
    fn id() -> i32 {
        return 0x0A
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(u8::from(self.difficulty).net_serialize());
        data.extend(self.difficulty_locked.net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

