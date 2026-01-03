use bevy::prelude::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;
use std::fmt::Debug;

pub trait OutgoingPacket: Sized + Message + Debug + Clone {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn serialize(&self) -> Option<OutgoingNetworkPacket>;

    fn to_packet(&self) -> Option<OutgoingNetworkPacket> {
        self.serialize()
    }
}
