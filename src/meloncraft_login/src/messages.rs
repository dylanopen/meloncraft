use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_player::{GameProfile, Locale, ClientViewDistance, ChatMode, ChatColors, DisplayedSkinParts, MainHand, EnableTextFiltering, AllowPlayerListings, ParticleRenderingMode};

#[derive(Message, Debug, Clone)]
pub struct OfflineLoggedIn {
    pub client: Entity,
    pub profile: GameProfile,
}

#[derive(Message, Debug, Clone)]
pub struct ClientLocaleReceived {
    pub client: Entity,
    pub locale: Locale,
}

#[derive(Message, Debug, Clone)]
pub struct ClientViewDistanceReceived {
    pub client: Entity,
    pub view_distance: ClientViewDistance,
}

#[derive(Message, Debug, Clone)]
pub struct ClientChatModeReceived {
    pub client: Entity,
    pub chat_mode: ChatMode,
}

#[derive(Message, Debug, Clone)]
pub struct ClientChatColorsReceived {
    pub client: Entity,
    pub chat_colors: ChatColors,
}

#[derive(Message, Debug, Clone)]
pub struct ClientDisplayedSkinPartsReceived {
    pub client: Entity,
    pub displayed_skin_parts: DisplayedSkinParts,
}

#[derive(Message, Debug, Clone)]
pub struct ClientMainHandReceived {
    pub client: Entity,
    pub main_hand: MainHand,
}

#[derive(Message, Debug, Clone)]
pub struct ClientEnableTextFilteringReceived {
    pub client: Entity,
    pub enable_text_filtering: EnableTextFiltering,
}

#[derive(Message, Debug, Clone)]
pub struct ClientAllowPlayerListingsReceived {
    pub client: Entity,
    pub allow_player_listings: AllowPlayerListings,
}

#[derive(Message, Debug, Clone)]
pub struct ClientParticleRenderingModeReceived {
    pub client: Entity,
    pub particle_rendering_mode: ParticleRenderingMode,
}
