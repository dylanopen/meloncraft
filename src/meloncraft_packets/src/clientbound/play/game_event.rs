use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::game_event::GameEventType;
use meloncraft_protocol_types::ProtocolType as _;

use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundGameEvent {
    pub client: Entity,
    pub event: GameEventType,
}

impl ClientboundPacket for ClientboundGameEvent {
    fn id() -> i32 {
        return 0x26;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.event.net_serialize());
    }
}
