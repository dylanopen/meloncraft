use bevy::prelude::Query;
use meloncraft_client::connection::ClientConnection;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::IncomingNetworkPacket;

pub trait IncomingPacket: Sized {
    fn id() -> i32;
    fn state() -> ConnectionState;
    fn parse(incoming: &IncomingNetworkPacket) -> Option<Self>;

    fn validate(
        incoming: &IncomingNetworkPacket,
        client_connections: &Query<&ClientConnection>,
    ) -> Result<(), ()> {
        let Ok(client_connection) = client_connections.get(incoming.client) else {
            return Err(());
        };
        if client_connection.state != Self::state() {
            return Err(());
        }
        if incoming.id != Self::id() {
            return Err(());
        }
        Ok(())
    }

    fn from_packet(
        incoming: &IncomingNetworkPacket,
        client_connections: &Query<&ClientConnection>,
    ) -> Option<Self> {
        Self::validate(&incoming, client_connections).ok()?;
        Self::parse(incoming)
    }
}
