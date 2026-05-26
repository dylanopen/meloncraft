use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_player::experience::{self, Experience};
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

use crate::ClientboundPacket;

#[derive(Message, Debug, Clone)]
pub struct ClientboundSetExperience {
    pub client: Entity,

    /// The *total* number of experience points a player has, see [`Experience`].
    pub total: Experience,

    /// The level shown to the player in the experience bar.
    /// If `None`, this will be calculated from [`ClientboundSetExperience::total`] (see
    /// [`experience::total_to_level`].
    pub level: Option<i32>,

    /// The fraction of the experience bar that is filled.
    /// Between `0.0` and `1.0`, with larger values indicating that more of the bar is filled.
    pub bar: Option<f32>,
}

impl ClientboundPacket for ClientboundSetExperience {
    fn id() -> i32 {
        return 0x65;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    #[expect(clippy::as_conversions, clippy::modulo_arithmetic, clippy::cast_possible_truncation, reason = "Both are needed for calculating experience level and bar size")]
    fn data(&self, data: &mut Vec<u8>) {
        data.extend(VarInt(self.total.0).net_serialize());
        let level = self.level.map_or_else(|| return experience::total_to_level(self.total.0) as i32, |level| return level);
        data.extend(VarInt(level).net_serialize());
        let bar = self.bar.map_or_else(|| return experience::total_to_level(self.total.0) % 1.0, |bar| return bar);
        data.extend(bar.net_serialize());
    }
}

