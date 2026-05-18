use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use meloncraft_text::JsonText;

#[derive(Message, Debug, Clone)]
pub struct ClientboundLoginDisconnect {
    pub client: Entity,
    pub reason: JsonText,
}

impl ClientboundPacket for ClientboundLoginDisconnect {
    fn id() -> i32 {
        return 0x00
    }

    fn state() -> ConnectionState {
        return ConnectionState::Login
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.reason.net_serialize(),
        })
    }
}
