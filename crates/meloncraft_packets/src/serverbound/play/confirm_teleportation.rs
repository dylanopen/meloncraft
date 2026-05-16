use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ServerboundConfirmTeleportation {
    pub client: Entity,
    pub teleport_id: i32,
}

impl ServerboundPacket for ServerboundConfirmTeleportation {
    fn id() -> i32 {
        return 0x00
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;
        let teleport_id = VarInt::net_deserialize(&mut incoming.data).unwrap().0;
        return Some(Self { client, teleport_id })
    }
}
