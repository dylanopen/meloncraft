use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;

#[derive(Clone, Debug)]
pub struct ServerboundNetworkPacket {
    pub client: Entity,
    pub len: i32,
    pub id: i32,
    pub data: Vec<u8>,
    pub state: ConnectionState,
}

#[derive(Message)]
pub struct ServerboundNetworkPacketReceived {
    pub packet: ServerboundNetworkPacket,
}

#[derive(Clone, Debug)]
pub struct ClientboundNetworkPacket {
    pub client: Entity,
    pub id: i32,
    pub data: Vec<u8>,
}

#[derive(Message)]
pub struct ClientboundNetworkPacketReceived {
    pub packet: ClientboundNetworkPacket,
}
