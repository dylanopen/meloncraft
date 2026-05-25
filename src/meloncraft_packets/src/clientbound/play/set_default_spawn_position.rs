use bevy::ecs::message::Message;
use bevy::math::DVec3 ;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

/// Sent by the server after login to specify the coordinates of the spawn point (the point at
/// which players spawn at, and which the compass points to).
///
/// It can be sent at any time to update the point compasses point at.
///
/// Before receiving this packet, the client uses the default position 8, 64, 8, and angle 0.0. 
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetDefaultSpawnPosition {
    pub client: Entity,

    /// Name of spawn dimension, e.g. `minecraft:overworld`.
    pub dimension: String,

    /// The coordinates of the spawn position, as a [`DVec3`] (3 doubles).
    pub location: DVec3,

    pub yaw: f32,
    pub pitch: f32,
}

impl ClientboundPacket for ClientboundSetDefaultSpawnPosition {
    fn id() -> i32 {
        return 0x5F;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(self.dimension.net_serialize());
        data.extend(self.location.net_serialize());
        data.extend(self.yaw.net_serialize());
        data.extend(self.pitch.net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

