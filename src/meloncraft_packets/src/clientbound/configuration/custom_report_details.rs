use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::DisconnectReport;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType as _};

#[derive(Message, Debug, Clone)]
pub struct ClientboundCustomReportDetails {
    pub client: Entity,
    pub report: Vec<DisconnectReport>,
}

impl ClientboundPacket for ClientboundCustomReportDetails {
    fn id() -> i32 {
        return 0x0F;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(PrefixedArray(self.report.clone()).net_serialize());
    }
}
