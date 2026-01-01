use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use crate::IncomingPacket;

#[derive(Message, Debug)]
pub struct StatusRequest {
    pub client: Entity,
}

impl IncomingPacket for StatusRequest {
    fn id() -> i32 { 0x00 }
    fn state() -> ConnectionState { ConnectionState::Status}
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        dbg!();
        Some(StatusRequest {
            client: incoming.client,
        })
    }
}
