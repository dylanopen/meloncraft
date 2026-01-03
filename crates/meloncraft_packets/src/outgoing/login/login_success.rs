use crate::outgoing_packet::OutgoingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;
use meloncraft_protocol_types::{GameProfile, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct LoginSuccess {
    pub client: Entity,
    pub game_profile: GameProfile,
}

impl OutgoingPacket for LoginSuccess {
    fn id() -> i32 {
        0x02
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn serialize(&self) -> Option<OutgoingNetworkPacket> {
        Some(OutgoingNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.game_profile.net_serialize(),
        })
    }
}
