use crate::ServerboundPacket;
use crate::network_messages::ServerboundNetworkPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::{Byte, PrefixedArray, ProtocolBuffer as _};

#[derive(Message, Debug, Clone)]
pub struct ServerboundEncryptionResponse {
    pub client: Entity,
    pub shared_secret: Vec<Byte>, // bytes are stored as i8s in the
    pub verify_token: Vec<Byte>,  // mc protocol for some reason
}

impl ServerboundPacket for ServerboundEncryptionResponse {
    fn id() -> i32 {
        return 0x01;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Login;
    }

    fn deserialize(packet: ServerboundNetworkPacket) -> Option<Self> {
        let mut incoming = packet;
        let shared_secret: PrefixedArray<Byte> = incoming.data.net_deserialize().unwrap();
        let shared_secret = shared_secret.0;
        let verify_token: PrefixedArray<Byte> = incoming.data.net_deserialize().unwrap();
        let verify_token = verify_token.0;
        return Some(Self {
            client: incoming.client,
            shared_secret,
            verify_token,
        });
    }
}
