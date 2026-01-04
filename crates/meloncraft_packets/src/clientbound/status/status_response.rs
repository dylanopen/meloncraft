use crate::clientbound_packet::ClientboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct StatusResponse {
    pub client: Entity,
    pub version_name: String,
    pub version_protocol: u32,
    pub max_players: u32,
    pub online_players: u32,
    pub description: String,
    pub enforces_secure_chat: bool,
}

impl ClientboundPacket for StatusResponse {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Status
    }
    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let json = format!(
            "{{\"version\": {{\"name\": \"{}\",\"protocol\": {}}},\"players\": {{\"max\": {},\"online\": {},\"sample\": []}},\"description\": {{\"text\": \"{}\"}},\"enforcesSecureChat\": {}}}",
            self.version_name,
            self.version_protocol,
            self.max_players,
            self.online_players,
            self.description,
            self.enforces_secure_chat,
        );
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: json.net_serialize(),
        })
    }
}
