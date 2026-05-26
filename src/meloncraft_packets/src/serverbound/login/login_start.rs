use crate::ServerboundPacket;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ServerboundNetworkPacket;
use meloncraft_player::Uuid;
use meloncraft_protocol_types::ProtocolType as _;

#[derive(Message, Debug, Clone)]
pub struct ServerboundLoginStart {
    pub client: Entity,
    pub name: String,
    pub uuid: Uuid,
}

impl ServerboundPacket for ServerboundLoginStart {
    fn id() -> i32 {
        return 0x00
    }
    fn state() -> ConnectionState {
        return ConnectionState::Login
    }

    fn deserialize(packet: ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet.clone();
        let client = incoming.client;
        let name = String::net_deserialize(&mut incoming.data).unwrap();
        let uuid = Uuid::net_deserialize(&mut incoming.data).unwrap();
        return Some(Self { client, name, uuid })
    }
}
