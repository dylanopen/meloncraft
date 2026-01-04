use crate::ServerboundPacket;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::Message;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_network::packet::ServerboundNetworkPacket;
use meloncraft_player::{
    AllowPlayerListings, ChatColors, ChatMode, DisplayedSkinParts, EnableTextFiltering, Locale,
    MainHand, ParticleRenderingMode, ViewDistance,
};
use meloncraft_protocol_types::{Byte, ProtocolBuffer, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ClientInformation {
    pub client: Entity,
    pub locale: Locale,
    pub view_distance: ViewDistance,
    pub chat_mode: ChatMode,
    pub chat_colors: ChatColors,
    pub displayed_skin_parts: DisplayedSkinParts,
    pub main_hand: MainHand,
    pub enable_text_filtering: EnableTextFiltering,
    pub allow_player_listings: AllowPlayerListings,
    pub particle_rendering_mode: ParticleRenderingMode,
}

impl ServerboundPacket for ClientInformation {
    fn id() -> i32 {
        0x00
    }
    fn state() -> ConnectionState {
        ConnectionState::Configuration
    }

    fn deserialize(packet: &ServerboundNetworkPacket) -> Option<Self> {
        let mut data = packet.data.clone();
        let client = packet.client;
        let locale = Locale(data.net_deserialize().unwrap());
        let view_distance: Byte = data.net_deserialize().unwrap();
        let view_distance = ViewDistance(view_distance.0 as u8);
        let chat_mode: VarInt = data.net_deserialize().unwrap();
        let chat_mode = ChatMode::try_from(chat_mode.0).unwrap();
        let chat_colors: bool = data.net_deserialize().unwrap();
        let chat_colors = ChatColors(chat_colors);
        let displayed_skin_parts = data.net_deserialize().unwrap();
        let main_hand: VarInt = data.net_deserialize().unwrap();
        let main_hand = MainHand::try_from(main_hand.0).unwrap();
        let enable_text_filtering = EnableTextFiltering(data.net_deserialize().unwrap());
        let allow_player_listings = AllowPlayerListings(data.net_deserialize().unwrap());
        let particle_rendering_mode: VarInt = data.net_deserialize().unwrap();
        let particle_rendering_mode =
            ParticleRenderingMode::try_from(particle_rendering_mode.0).unwrap();

        Some(Self {
            client,
            locale,
            view_distance,
            chat_mode,
            chat_colors,
            displayed_skin_parts,
            main_hand,
            enable_text_filtering,
            allow_player_listings,
            particle_rendering_mode,
        })
    }
}
