//! Module for [`MeloncraftPlayerStatePlugin`].

use crate::client_action;
use crate::gamemode;
use crate::movement;
use bevy::app::{App, Plugin, Update};

pub struct MeloncraftPlayerStatePlugin;

/// Meloncraft plugin to handle player events and packets, and then store them as components on
/// the player's Bevy Entity.
/// For example, this responds to serverbound player movement packets, and stores the player's
/// position in the ECS in a component, so that other systems can query their position later.
impl Plugin for MeloncraftPlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::fwd_player_moved);
        app.add_systems(Update, movement::fwd_player_teleport);

        app.add_systems(
            Update,
            (
                client_action::send_client_player_action,
                client_action::send_add_player,
            ),
        );

        app.add_systems(
            Update,
            (
                gamemode::send_gamemode_info_update,
                gamemode::send_gamemode_game_event,
            ),
        );
    }
}
