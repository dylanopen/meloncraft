use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use meloncraft_protocol_types::deserialize;
use crate::IncomingPacket;

#[derive(Message, Debug)]
pub struct StatusRequest {}

impl IncomingPacket for StatusRequest {
    fn id() -> i32 { 0x00 }
    fn state() -> ConnectionState { ConnectionState::Handshaking }
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        None
    }
}
