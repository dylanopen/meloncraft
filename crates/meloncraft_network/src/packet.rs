use bevy::prelude::{Entity, Message};

#[derive(Clone, Debug)]
pub struct IncomingNetworkPacket {
    pub client: Entity,
    pub len: i32,
    pub id: i32,
    pub data: Vec<u8>,
}

#[derive(Message)]
pub struct IncomingNetworkPacketReceived {
    pub packet: IncomingNetworkPacket,
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
