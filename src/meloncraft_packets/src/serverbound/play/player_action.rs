use crate::ServerboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_block::face::BlockFaceType;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_player::PlayerActionStatus;
use meloncraft_protocol_types::{NetworkLocation, ProtocolType as _, ProtocolBuffer as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ServerboundPlayerAction {
    pub client: Entity,
    pub status: PlayerActionStatus,
    pub block_location: NetworkLocation,
    pub block_face: BlockFaceType,
    pub sequence: i32,
}

impl ServerboundPacket for ServerboundPlayerAction {
    fn id() -> i32 {
        return 0x28
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;

        let status = VarInt::net_deserialize(&mut incoming.data).ok()?.0.try_into().unwrap();
        let block_location = incoming.data.net_deserialize().ok()?;
        let block_face = u8::net_deserialize(&mut incoming.data).ok()?.try_into().ok()?;
        let sequence = VarInt::net_deserialize(&mut incoming.data).ok()?.0;

        return Some(Self {
            client,
            status,
            block_location,
            block_face,
            sequence,
        })
    }
}
