use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType, VarInt};

use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundSynchronizePlayerPosition {
    pub client: Entity,
    pub teleport_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl ClientboundPacket for ClientboundSynchronizePlayerPosition {
    fn id() -> i32 {
        0x46
    }

    fn state() -> ConnectionState {
        ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();
        data.extend(VarInt(self.teleport_id).net_serialize());
        data.extend(self.x.net_serialize());
        data.extend(self.y.net_serialize());
        data.extend(self.z.net_serialize());
        data.extend(self.velocity_x.net_serialize());
        data.extend(self.velocity_y.net_serialize());
        data.extend(self.velocity_z.net_serialize());
        data.extend(self.yaw.net_serialize());
        data.extend(self.pitch.net_serialize());
        data.extend(0i32.net_serialize()); // teleport flags, I think 0 will work fine?
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

