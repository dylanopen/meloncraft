use crate::messages::{
    ClientAllowPlayerListingsReceived, ClientChatColorsReceived, ClientChatModeReceived,
    ClientDisplayedSkinPartsReceived, ClientEnableTextFilteringReceived, ClientLocaleReceived,
    ClientMainHandReceived, ClientParticleRenderingModeReceived, ClientViewDistanceReceived,
};
use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_packets::ServerboundClientInformation;

#[expect(
    clippy::too_many_arguments,
    reason = "All args necessary, they're just message writers"
)]
pub fn client_information_listener(
    mut login_acknowledged_pr: MessageReader<ServerboundClientInformation>,
    mut locale_mw: MessageWriter<ClientLocaleReceived>,
    mut view_distance_mw: MessageWriter<ClientViewDistanceReceived>,
    mut chat_mode_mw: MessageWriter<ClientChatModeReceived>,
    mut chat_colors_mw: MessageWriter<ClientChatColorsReceived>,
    mut displayed_skin_parts_mw: MessageWriter<ClientDisplayedSkinPartsReceived>,
    mut main_hand_mw: MessageWriter<ClientMainHandReceived>,
    mut enable_text_filtering_mw: MessageWriter<ClientEnableTextFilteringReceived>,
    mut allow_player_listings_mw: MessageWriter<ClientAllowPlayerListingsReceived>,
    mut particle_rendering_mode_mw: MessageWriter<ClientParticleRenderingModeReceived>,
) {
    for packet in login_acknowledged_pr.read() {
        let client = packet.client;

        locale_mw.write(ClientLocaleReceived {
            client,
            locale: packet.locale.clone(),
        });
        view_distance_mw.write(ClientViewDistanceReceived {
            client,
            view_distance: packet.view_distance,
        });
        chat_mode_mw.write(ClientChatModeReceived {
            client,
            chat_mode: packet.chat_mode,
        });
        chat_colors_mw.write(ClientChatColorsReceived {
            client,
            chat_colors: packet.chat_colors,
        });
        displayed_skin_parts_mw.write(ClientDisplayedSkinPartsReceived {
            client,
            displayed_skin_parts: packet.displayed_skin_parts.clone(),
        });
        main_hand_mw.write(ClientMainHandReceived {
            client,
            main_hand: packet.main_hand,
        });
        enable_text_filtering_mw.write(ClientEnableTextFilteringReceived {
            client,
            enable_text_filtering: packet.enable_text_filtering,
        });
        allow_player_listings_mw.write(ClientAllowPlayerListingsReceived {
            client,
            allow_player_listings: packet.allow_player_listings,
        });
        particle_rendering_mode_mw.write(ClientParticleRenderingModeReceived {
            client,
            particle_rendering_mode: packet.particle_rendering_mode.clone(),
        });
    }
}
