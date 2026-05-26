use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};
use meloncraft_server_info::world_border::WorldBorderWarningDistance;

/// Set the minimum distance a player needs to be to the world border, until the red outline on
/// their screen, warning them that they are too close to the world border.
/// See [`WorldBorderWarningDistance`].
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetBorderWarningDistance {
    pub client: Entity,

    /// Set the minimum distance a player needs to be to the world border, until the red outline on
    /// their screen, warning them that they are too close to the world border.
    /// See [`WorldBorderWarningDistance`].
    pub warning_distance: WorldBorderWarningDistance,
}

impl ClientboundPacket for ClientboundSetBorderWarningDistance {
    fn id() -> i32 {
        return 0x5A;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(VarInt(self.warning_distance.0).net_serialize());
    }
}
