use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::pause_menu::PauseMenuLink;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{PrefixedArray, ProtocolType as _};

#[derive(Message, Debug, Clone)]
pub struct ClientboundServerLinks {
    pub client: Entity,
    pub links: Vec<PauseMenuLink>,
}

impl ClientboundPacket for ClientboundServerLinks {
    fn id() -> i32 {
        return 0x10
    }

    fn state() -> ConnectionState {
        return ConnectionState::Configuration
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let data = PrefixedArray(self.links.clone()).net_serialize();
        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
