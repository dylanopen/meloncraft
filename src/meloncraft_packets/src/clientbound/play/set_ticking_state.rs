use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_server_info::tick::{TickRate, TickingFrozen};
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

/// Used to adjust the ticking rate of the client, and whether it's frozen. 
///
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Ticking_State>.
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetTickingState {
    pub client: Entity,

    /// The target ticking rate of the server, see [`TickRate`].
    pub tick_rate: TickRate,

    /// Whether or not the client should treat the tick rate as 'frozen', i.e. no automatic ticks,
    /// the server has to send a packet for every tick. See [`TickingFrozen`].
    pub ticking_frozen: TickingFrozen,
}

impl ClientboundPacket for ClientboundSetTickingState {
    fn id() -> i32 {
        return 0x7D;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(self.tick_rate.0.net_serialize());
        data.extend(self.ticking_frozen.0.net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

