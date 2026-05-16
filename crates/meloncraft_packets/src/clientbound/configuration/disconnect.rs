use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use meloncraft_text::JsonText;

#[derive(Message, Clone, Debug)]
pub struct ClientboundConfigurationDisconnect {
    pub client: Entity,
    pub reason: JsonText,
}

impl ClientboundPacket for ClientboundConfigurationDisconnect {
    fn id() -> i32 {
        return 0x02
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data: self.reason.net_serialize(),
        })
    }
}
