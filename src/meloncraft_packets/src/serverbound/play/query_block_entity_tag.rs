use crate::ServerboundPacket;
use crate::network_messages::ServerboundNetworkPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::{NetworkLocation, ProtocolType as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ServerboundQueryBlockEntityTag {
    pub client: Entity,
    pub transaction_id: i32,
    pub location: NetworkLocation,
}

impl ServerboundPacket for ServerboundQueryBlockEntityTag {
    fn id() -> i32 {
        return 0x01;
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }
    fn deserialize(packet: ServerboundNetworkPacket) -> Option<Self> {
        let mut packet = packet;
        let client = packet.client;

        let transaction_id = VarInt::net_deserialize(&mut packet.data).unwrap().0;
        let location = NetworkLocation::net_deserialize(&mut packet.data).ok()?;

        return Some(Self {
            client,
            transaction_id,
            location,
        });
    }
}
