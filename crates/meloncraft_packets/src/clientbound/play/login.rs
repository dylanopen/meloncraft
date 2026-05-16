use crate::clientbound_packet::ClientboundPacket;
use bevy::prelude::{Entity, Message};
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_core::Identifier;
use meloncraft_network::packet::ClientboundNetworkPacket;
use meloncraft_protocol_types::{NetworkLocation, PrefixedArray, ProtocolType as _, VarInt};

#[expect(clippy::struct_excessive_bools, reason = "It's a packet: we need all those bools, and it's not a state machine.")]
#[derive(Message, Debug, Clone)]
pub struct ClientboundPlayLogin {
    pub client: Entity,
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: PrefixedArray<Identifier>,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: i32,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub death_dimension_name: Option<Identifier>,
    pub death_location: Option<NetworkLocation>,
    pub portal_cooldown: i32,
    pub sea_level: i32,
    pub enforces_secure_chat: bool,
}

impl ClientboundPacket for ClientboundPlayLogin {
    fn id() -> i32 {
        return 0x30
    }
    fn state() -> ConnectionState {
        return ConnectionState::Play
    }
    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();
        data.extend(self.entity_id.net_serialize());
        data.extend(self.is_hardcore.net_serialize());
        data.extend(self.dimension_names.net_serialize());
        data.extend(VarInt(self.max_players).net_serialize());
        data.extend(VarInt(self.view_distance).net_serialize());
        data.extend(VarInt(self.simulation_distance).net_serialize());
        data.extend(self.reduced_debug_info.net_serialize());
        data.extend(self.enable_respawn_screen.net_serialize());
        data.extend(self.do_limited_crafting.net_serialize());
        data.extend(VarInt(self.dimension_type).net_serialize());
        data.extend(self.dimension_name.net_serialize());
        data.extend(self.hashed_seed.net_serialize());
        data.extend(self.game_mode.net_serialize());
        data.extend(self.previous_game_mode.net_serialize());
        data.extend(self.is_debug.net_serialize());
        data.extend(self.is_flat.net_serialize());
        data.extend(self.has_death_location.net_serialize());
        if self.has_death_location {
            // maybe we should serialize this as optionals, or maybe not? I don't know.
            data.extend(self.death_dimension_name.net_serialize());
            data.extend(self.death_location.net_serialize());
        }
        data.extend(VarInt(self.portal_cooldown).net_serialize());
        data.extend(VarInt(self.sea_level).net_serialize());
        data.extend(self.enforces_secure_chat.net_serialize());
        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}
