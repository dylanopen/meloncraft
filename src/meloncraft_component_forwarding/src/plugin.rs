//! Module for [`MeloncraftComponentForwardingPlugin`].

use bevy::app::{App, Plugin, Update};

use crate::{abilities, bossbar, experience, health};

/// Registers systems to send components to clients in packets, when the components change or players
/// join.
///
/// Add this plugin if you want to be able to just add and modify components of entities in your app and have
/// them automatically sent to clients without you needing to think about it.
pub struct MeloncraftComponentForwardingPlugin;

impl Plugin for MeloncraftComponentForwardingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, health::send_health);
        app.add_systems(Update, abilities::send_player_abilities);
        app.add_systems(Update, experience::send_player_experience);
        app.add_systems(Update, bossbar::send_bossbar_on_change_active);
    }
}
