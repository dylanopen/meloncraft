use crate::IncomingPacket;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::ProtocolType;

#[derive(Message, Debug, Clone)]
pub struct LoginStart {
    pub client: Entity,
    pub name: String,
    pub uuid: Uuid,
}

impl IncomingPacket for LoginStart {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        let mut incoming = incoming.clone();
        let client = incoming.client;
        let name = String::net_deserialize(&mut incoming.data).unwrap();
        let uuid = Uuid::net_deserialize(&mut incoming.data).unwrap();
        Some(Self { client, uuid, name })
    }
}
