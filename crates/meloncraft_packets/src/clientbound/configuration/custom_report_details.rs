use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::DisconnectReport;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType};

#[derive(Message, Debug, Clone)]
pub struct CustomReportDetails {
    pub client: Entity,
    pub report: Vec<DisconnectReport>,
}

impl ClientboundPacket for CustomReportDetails {
    fn id() -> i32 {
        0x0F
    }

    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let data = PrefixedArray(self.report.clone()).net_serialize();
        Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
