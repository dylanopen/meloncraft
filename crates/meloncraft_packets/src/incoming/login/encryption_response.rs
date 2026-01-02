use crate::IncomingPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;
use meloncraft_protocol_types::{Byte, PrefixedArray, ProtocolBuffer};

#[derive(Message, Debug, Clone)]
pub struct EncryptionResponse {
    pub client: Entity,
    pub shared_secret: Vec<Byte>, // bytes are stored as i8s in the
    pub verify_token: Vec<Byte>,  // mc protocol for some reason
}

impl IncomingPacket for EncryptionResponse {
    fn id() -> i32 {
        0x01
    }

    fn state() -> ConnectionState {
        ConnectionState::Login
    }

    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self> {
        let mut incoming = incoming.clone();
        let shared_secret: PrefixedArray<Byte> = incoming.data.net_deserialize().unwrap();
        let shared_secret = shared_secret.values;
        let verify_token: PrefixedArray<Byte> = incoming.data.net_deserialize().unwrap();
        let verify_token = verify_token.values;
        Some(Self {
            client: incoming.client,
            shared_secret,
            verify_token,
        })
    }
}
