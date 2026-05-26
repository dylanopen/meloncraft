use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_server_info::world_border::WorldBorderWarningDelay;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};
use crate::clientbound_packet::ClientboundPacket;

/// Set the time in seconds before a player receives the red outline on their screen, warning them that they
/// are too close to the world border. See [`WorldBorderWarningDelay`].
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetBorderWarningDelay {
    pub client: Entity,

    /// Set the time in seconds before a player receives the red outline on their screen, warning them
    /// that they are too close to the world border. See [`WorldBorderWarningDelay`].
    pub warning_delay: WorldBorderWarningDelay,
}

impl ClientboundPacket for ClientboundSetBorderWarningDelay {
    fn id() -> i32 {
        return 0x59
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {

        data.extend(VarInt(self.warning_delay.0).net_serialize());

    }
}

