use crate::IncomingPacket;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::deserialize;

#[derive(Message)]
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
        let name = deserialize::string(&mut incoming.data).unwrap();
        let uuid = deserialize::uuid(&mut incoming.data).unwrap();
        let uuid = Uuid(uuid);
        Some(Self { client, uuid, name })
    }
}
