use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_nbt::{NbtString, NbtTag, NbtValue};
use meloncraft_server_info::icon::ServerIcon;
use meloncraft_server_info::motd::Motd;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType as _};
use crate::clientbound_packet::ClientboundPacket;

/// Send an MOTD and server icon to the client, but during the `play` connection state.
#[derive(Message, Debug, Clone)]
pub struct ClientboundServerData {
    pub client: Entity,

    /// The server's message of the day (description).
    pub motd: Motd,

    /// A byte array, storing the bytes of **a PNG formatted image**.
    pub icon: Option<ServerIcon>,
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

        data.extend(NbtTag::new(String::new(), NbtValue::String(NbtString(self.motd.0.clone()))).net_serialize());
        data.extend(self.icon.clone()
            .map(|icon_data| return PrefixedArray(icon_data.0))
            .net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

