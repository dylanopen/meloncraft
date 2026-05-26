use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_server_info::world_border::WorldBorderDiameter;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

/// Set the size of the active worldborder to the specified [`WorldBorderDiameter`].
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetBorderSize {
    pub client: Entity,

    /// The diameter of the world border, see [`WorldBorderDiameter`].
    pub diameter: WorldBorderDiameter,
}

impl ClientboundPacket for ClientboundSetBorderSize {
    fn id() -> i32 {
        return 0x58
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(self.diameter.0.net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

