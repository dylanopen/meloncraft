use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ServerboundBundleItemSelected {
    pub client: Entity,
    pub inventory_slot: i32,
    pub bundle_slot: i32,
}

impl ServerboundPacket for ServerboundBundleItemSelected {
    fn id() -> i32 {
        return 0x02
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;

        let inventory_slot = VarInt::net_deserialize(&mut incoming.data).unwrap().0;
        let bundle_slot = VarInt::net_deserialize(&mut incoming.data).unwrap().0

        return Some(Self {
            client,
            inventory_slot,
            bundle_slot
        })
    }
}
