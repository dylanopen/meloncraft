use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_protocol_types::ProtocolType as _;

/// Set the amount of time that title messages take to fade in, stay on the screen, and
/// fade out.
///
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Title_Animation_Times>.
#[derive(Message, Debug, Clone)]
pub struct ClientboundSetTitleAnimationTimes {
    pub client: Entity,

    /// The number of ticks that it takes for any titles to fade *in* on the client's screen.
    pub fade_in_ticks: i32,

    /// The number of ticks that the title message should be fully shown for.
    pub stay_ticks: i32,

    /// The number of ticks that it takes for any titles to fade *out* on the client's screen.
    pub fade_out_ticks: i32,
}

impl ClientboundPacket for ClientboundSetTitleAnimationTimes {
    fn id() -> i32 {
        return 0x71;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.fade_in_ticks.net_serialize());
        data.extend(self.stay_ticks.net_serialize());
        data.extend(self.fade_out_ticks.net_serialize());
    }
}
