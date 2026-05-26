use bevy::ecs::message::Message;
use bevy::math::IVec3;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{NetworkLocation, ProtocolType as _};
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

    /// The coordinates of the spawn position, as an [`IVec3`] (3 ints).
    /// When serializing, this is converted into a [`NetworkLocation`].
    pub location: IVec3,

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


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {

        data.extend(self.dimension.net_serialize());
        data.extend(NetworkLocation(self.location).net_serialize());
        data.extend(self.yaw.net_serialize());
        data.extend(self.pitch.net_serialize());

    }
}

