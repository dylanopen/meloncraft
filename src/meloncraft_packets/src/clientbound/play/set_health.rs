use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_entity::health::food::{FoodHealth, FoodSaturation};
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};
use crate::clientbound_packet::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundSetHealth {
    pub client: Entity,
    pub current: CurrentHealth,
    pub food: FoodHealth,
    pub saturation: FoodSaturation,
}

impl ClientboundPacket for ClientboundSetHealth {
    fn id() -> i32 {
        return 0x66
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {

        data.extend(self.current.0.net_serialize());
        data.extend(VarInt(self.food.0).net_serialize());
        data.extend(self.saturation.0.net_serialize());

    }
}

