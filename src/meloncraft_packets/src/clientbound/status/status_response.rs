use crate::clientbound_packet::ClientboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ClientboundStatusResponse {
    pub client: Entity,
    pub version_name: String,
    pub version_protocol: u32,
    pub max_players: u32,
    pub online_players: u32,
    pub description: String,
    pub enforces_secure_chat: bool,
}

impl ClientboundPacket for ClientboundStatusResponse {
    fn id() -> i32 {
        return 0x00;
    }
    fn state() -> ConnectionState {
        return ConnectionState::Status;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        let json: String = format!(
            "{{\"version\": {{\"name\": \"{}\",\"protocol\": {}}},\"players\": {{\"max\": {},\"online\": {},\"sample\": []}},\"description\": {{\"text\": \"{}\"}},\"enforcesSecureChat\": {}}}",
            self.version_name,
            self.version_protocol,
            self.max_players,
            self.online_players,
            self.description,
            self.enforces_secure_chat,
        );
        data.extend(json.net_serialize());
    }
}
