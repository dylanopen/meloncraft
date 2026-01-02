use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::OutgoingNetworkPacket;

pub trait OutgoingPacket: Sized {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn serialize(&self) -> Option<OutgoingNetworkPacket>;

    fn to_packet(&self) -> Option<OutgoingNetworkPacket> {
        self.serialize()
    }
}
