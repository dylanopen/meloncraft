use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_server_info::motd::Motd;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

/// Send an MOTD and server icon to the client, but during the `play` connection state.
#[derive(Message, Debug, Clone)]
pub struct ClientboundServerData {
    pub client: Entity,

    /// The server's message of the day (description).
    pub motd: Motd,

    /// A byte array, storing the bytes of **a PNG formatted image**.
    pub icon: Vec<u8>,
}

impl ClientboundPacket for ClientboundServerData {
    fn id() -> i32 {
        return 0x54
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(self.motd.0.net_serialize());
        data.extend(self.icon.clone());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

