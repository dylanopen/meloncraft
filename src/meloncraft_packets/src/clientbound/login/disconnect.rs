use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;
use meloncraft_text::SnbtText;

#[derive(Message, Debug, Clone)]
pub struct ClientboundLoginDisconnect {
    pub client: Entity,
    pub reason: SnbtText,
}

impl ClientboundPacket for ClientboundLoginDisconnect {
    fn id() -> i32 {
        return 0x00;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Login;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.reason.net_serialize());
    }
}
