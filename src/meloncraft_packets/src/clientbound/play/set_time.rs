use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_server_info::time::{DayTime, DaylightCycle, OpenTime};
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

/// Set the center of the active worldborder to the specified [`WorldBorderCenter`].
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetTime {
    pub client: Entity,

    /// The real-world time, in ticks, the server has run for (across all program runs).
    /// Also known as `World Age` in the protocol.
    /// See [`OpenTime`] for more info.
    pub open_time: OpenTime,

    /// The current game time, in ticks, see [`DayTime`] for more info.
    pub day_time: DayTime,

    /// Whether or not time is advancing, see [`DaylightCycle`] for information.
    pub daylight_cycle: DaylightCycle,
}

impl ClientboundPacket for ClientboundSetTime {
    fn id() -> i32 {
        return 0x6F
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend(self.open_time.0.net_serialize());
        data.extend(self.day_time.0.net_serialize());
        data.extend(self.daylight_cycle.0.net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

