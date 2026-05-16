use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_player::GameProfile;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ClientboundLoginSuccess {
    pub client: Entity,
    pub game_profile: GameProfile,
}

impl ClientboundPacket for ClientboundLoginSuccess {
    fn id() -> i32 {
        return 0x02
    }

    fn state() -> ConnectionState {
        return ConnectionState::Login
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.game_profile.net_serialize(),
        })
    }
}
