use bevy::prelude::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ClientboundNetworkPacket;
use std::fmt::Debug;

pub trait ClientboundPacket: Sized + Message + Debug + Clone {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn serialize(&self) -> Option<ClientboundNetworkPacket>;

    fn to_packet(&self) -> Option<ClientboundNetworkPacket> {
        self.serialize()
    }
}
