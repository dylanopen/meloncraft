use bevy::ecs::entity::Entity;
use bevy::prelude::Message;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use core::fmt::Debug;

pub trait ClientboundPacket: Sized + Message + Debug + Clone {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn client(&self) -> Entity;
    fn serialize(&self) -> Option<ClientboundNetworkPacket>;
}
