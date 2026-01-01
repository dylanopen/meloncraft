use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::{IncomingNetworkPacket, OutgoingNetworkPacket};

pub trait OutgoingPacket: Sized {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn serialize(&self) -> Option<OutgoingNetworkPacket>;

    fn validate(incoming: &IncomingNetworkPacket) -> Result<(), ()> {
        if incoming.state != Self::state() {
            return Err(());
        }
        if incoming.id != Self::id() {
            return Err(());
        }
        Ok(())
    }

    fn to_packet(&self) -> Option<OutgoingNetworkPacket> {
        self.serialize()
    }
}
