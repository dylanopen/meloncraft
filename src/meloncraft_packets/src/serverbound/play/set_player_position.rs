use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_entity_position::{EntityPosition, EntityPositionFlags};
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolBuffer as _, ProtocolType as _};

#[derive(Message, Debug, Clone)]
pub struct ServerboundSetPlayerPosition {
    pub client: Entity,
    pub position: EntityPosition,
}

impl ServerboundPacket for ServerboundSetPlayerPosition {
    fn id() -> i32 {
        return 0x1D
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;

        let x = incoming.data.net_deserialize().ok()?;
        let y = incoming.data.net_deserialize().ok()?;
        let z = incoming.data.net_deserialize().ok()?;
        let byte_flags = u8::net_deserialize(&mut incoming.data).ok()?;
        let flags = EntityPositionFlags::from(byte_flags);

        return Some(Self {
            client,
            position: EntityPosition { x, y, z, flags }
        })
    }
}
