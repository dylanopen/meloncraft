use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_server_info::world_border::WorldBorderCenter;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

/// Set the center of the active worldborder to the specified [`WorldBorderCenter`].
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetBorderCenter {
    pub client: Entity,

    /// The center of the world border, see [`WorldBorderCenter`].
    pub center: WorldBorderCenter,
}

impl ClientboundPacket for ClientboundSetBorderCenter {
    fn id() -> i32 {
        return 0x56
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {

        data.extend(self.center.0.net_serialize());

    }
}

