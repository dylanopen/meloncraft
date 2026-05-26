use bevy::ecs::entity::Entity;
use bevy::prelude::Message;
use meloncraft_client::connection_state::ConnectionState;
use crate::network_messages::ClientboundNetworkPacket;
use core::fmt::Debug;

pub trait ClientboundPacket: Sized + Message + Debug + Clone {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn client(&self) -> Entity;
    fn data(&self, data: &mut Vec<u8>);

    fn serialize(&self) -> ClientboundNetworkPacket {
        let mut body_buffer = Vec::new();
        self.data(&mut body_buffer);
        return ClientboundNetworkPacket {
            client: self.client(),
            id: Self::id(),
            data: body_buffer
        };
    }
}
