use bevy::prelude::Message;
use crate::connection_state::ConnectionState;

#[derive(Clone, Debug)]
pub struct IncomingNetworkPacket {
    pub len: i32,
    pub state: ConnectionState,
    pub id: i32,
    pub data: Vec<u8>,
}

#[derive(Message)]
pub struct IncomingNetworkPacketReceived {
    pub packet: IncomingNetworkPacket,
}


