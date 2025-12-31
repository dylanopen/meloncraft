use meloncraft_network::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;

pub trait IncomingPacket: Sized {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self>;

    fn validate(incoming: &IncomingNetworkPacket) -> Result<(), ()> {
        return Ok(())
    }

    fn from_packet(incoming: &IncomingNetworkPacket) -> Option<Self> {
        Self::validate(&incoming).ok()?;
        Self::parse(incoming)
    }
}
