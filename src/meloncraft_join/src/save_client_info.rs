use bevy::prelude::{Commands, MessageReader};
use meloncraft_login::messages::{
    ClientAllowPlayerListingsReceived, ClientChatColorsReceived, ClientChatModeReceived,
    ClientDisplayedSkinPartsReceived, ClientEnableTextFilteringReceived, ClientLocaleReceived,
    ClientMainHandReceived, ClientParticleRenderingModeReceived, ClientViewDistanceReceived,
};

pub fn allow_player_listings(
    mut reader: MessageReader<ClientAllowPlayerListingsReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.allow_player_listings);
    }
}

pub fn chat_colors(
    mut reader: MessageReader<ClientChatColorsReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.chat_colors);
    }
}

pub fn chat_mode(
    mut reader: MessageReader<ClientChatModeReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.chat_mode);
    }
}

pub fn displayed_skin_parts(
    mut reader: MessageReader<ClientDisplayedSkinPartsReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.displayed_skin_parts.clone());
    }
}

pub fn enable_text_filtering(
    mut reader: MessageReader<ClientEnableTextFilteringReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.enable_text_filtering);
    }
}

pub fn locale(
    mut reader: MessageReader<ClientLocaleReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.locale.clone());
    }
}

pub fn main_hand(
    mut reader: MessageReader<ClientMainHandReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.main_hand);
    }
}

pub fn particle_rendering_mode(
    mut reader: MessageReader<ClientParticleRenderingModeReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.particle_rendering_mode.clone());
    }
}

pub fn view_distance(
    mut reader: MessageReader<ClientViewDistanceReceived>,
    mut commands: Commands,
) {
    for message in reader.read() {
        commands.entity(message.client).insert(message.view_distance);
    }
}
