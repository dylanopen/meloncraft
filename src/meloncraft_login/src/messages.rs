use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_player::{GameProfile, Locale, ClientViewDistance, ChatMode, ChatColors};

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
