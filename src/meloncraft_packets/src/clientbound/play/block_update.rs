use bevy::ecs::message::Message;
use bevy::math::IVec3;
use bevy::prelude::Entity;
use meloncraft_block::block::Block;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{NetworkLocation, ProtocolType as _, VarInt};
use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundBlockUpdate {
    pub client: Entity,
    pub block_location: IVec3,
    pub new_block: Block,
}

impl ClientboundPacket for ClientboundBlockUpdate {
    fn id() -> i32 {
        return 0x08
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {

        data.extend(NetworkLocation(self.block_location).net_serialize());
        data.extend(VarInt(self.new_block.state_id).net_serialize());

    }
}

