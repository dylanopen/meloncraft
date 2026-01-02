use crate::outgoing_packet::OutgoingPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;
use meloncraft_protocol_types::serialize;

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

impl OutgoingPacket for StatusResponse {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Status
    }
    fn serialize(&self) -> Option<OutgoingNetworkPacket> {
        let json = format!(
            "{{\"version\": {{\"name\": \"{}\",\"protocol\": {}}},\"players\": {{\"max\": {},\"online\": {},\"sample\": []}},\"description\": {{\"text\": \"{}\"}},\"enforcesSecureChat\": {}}}",
            self.version_name,
            self.version_protocol,
            self.max_players,
            self.online_players,
            self.description,
            self.enforces_secure_chat,
        );
        Some(OutgoingNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: serialize::string(&json),
        })
    }
}
