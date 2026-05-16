mod state;
mod login_start;
mod save_client_info;

use bevy::app::{App, Plugin, Update};
use meloncraft_handshaking::MeloncraftHandshakingPlugin;

pub struct MeloncraftJoinPlugin;

impl Plugin for MeloncraftJoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeloncraftHandshakingPlugin);

        app.add_systems(Update, (
            login_start::save_profile,
            login_start::respond_success,
        ));
        app.add_systems(Update, (
            save_client_info::allow_player_listings,
            save_client_info::chat_colors,
            save_client_info::chat_mode,
            save_client_info::displayed_skin_parts,
            save_client_info::enable_text_filtering,
            save_client_info::locale,
            save_client_info::main_hand,
            save_client_info::particle_rendering_mode,
            save_client_info::view_distance,
        ));
    }
}