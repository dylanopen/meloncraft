use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::{PlayerAction, ProtocolType, VarInt};

use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundPlayerInfoUpdate {
    pub client: Entity,
    pub action_mask: u8,
    pub players: Vec<(Uuid, Vec<PlayerAction>)>,
}

impl ClientboundPacket for ClientboundPlayerInfoUpdate {
    fn id() -> i32 {
        0x44
    }

    fn state() -> ConnectionState {
        ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();
        data.push(self.action_mask); // 0xFF for all actions
        data.extend(VarInt(self.players.len().try_into().unwrap()).net_serialize());

        for player in &self.players {
            data.extend(player.0.net_serialize());
            for action in &player.1 {
                data.extend(action.net_serialize());
            }
        }

        Some(meloncraft_network::packet::ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

