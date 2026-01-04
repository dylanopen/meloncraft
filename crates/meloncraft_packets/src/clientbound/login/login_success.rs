use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_player::GameProfile;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct LoginSuccess {
    pub client: Entity,
    pub game_profile: GameProfile,
}

impl ClientboundPacket for LoginSuccess {
    fn id() -> i32 {
        0x02
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.game_profile.net_serialize(),
        })
    }
}
