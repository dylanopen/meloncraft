use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ConfirmTeleportation {
    pub client: Entity,
    pub teleport_id: i32,
}

impl ServerboundPacket for ConfirmTeleportation {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Play
    }
    fn deserialize(incoming: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = incoming.clone();
        let client = incoming.client;
        let teleport_id = VarInt::net_deserialize(&mut incoming.data).unwrap().0;
        Some(Self { client, teleport_id })
    }
}
