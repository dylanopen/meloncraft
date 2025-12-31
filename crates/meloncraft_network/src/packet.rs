use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;

#[derive(Clone, Debug)]
pub struct IncomingNetworkPacket {
    pub client: Entity,
    pub len: i32,
    pub state: ConnectionState,
    pub id: i32,
    pub data: Vec<u8>,
}

#[derive(Message)]
pub struct IncomingNetworkPacketReceived {
    pub packet: IncomingNetworkPacket,
}

#[derive(Clone, Debug)]
pub struct OutgoingNetworkPacket {
    pub client: Entity,
    pub id: i32,
    pub data: Vec<u8>,
}

#[derive(Message)]
pub struct OutgoingNetworkPacketReceived {
    pub packet: OutgoingNetworkPacket,
}
