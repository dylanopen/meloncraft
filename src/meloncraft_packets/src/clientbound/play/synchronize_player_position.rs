use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use bevy::math::DVec3;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundSynchronizePlayerPosition {
    pub client: Entity,
    pub teleport_id: i32,
    pub position: DVec3,
    pub velocity: DVec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl ClientboundPacket for ClientboundSynchronizePlayerPosition {
    fn id() -> i32 {
        return 0x46
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(VarInt(self.teleport_id).net_serialize());
        data.extend(self.position.net_serialize());
        data.extend(self.velocity.net_serialize());
        data.extend(self.yaw.net_serialize());
        data.extend(self.pitch.net_serialize());
        data.extend(0_i32.net_serialize()); // teleport flags, I think 0 will work fine?
    }
}

