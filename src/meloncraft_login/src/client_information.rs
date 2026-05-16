use bevy::prelude::{Commands, MessageReader};
use meloncraft_packets::ServerboundClientInformation;

pub fn client_information_listener(
    mut login_acknowledged_pr: MessageReader<ServerboundClientInformation>,
    mut commands: Commands,
) {
    for packet in login_acknowledged_pr.read() {
        let mut entity = commands.entity(packet.client);
        entity.insert((
            packet.locale.clone(),
            packet.view_distance,
            packet.chat_mode,
            packet.chat_colors,
            packet.displayed_skin_parts.clone(),
            packet.main_hand,
            packet.enable_text_filtering,
            packet.allow_player_listings,
            packet.particle_rendering_mode.clone(),
        ));
    }
}
