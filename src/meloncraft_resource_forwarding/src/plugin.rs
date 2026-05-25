//! Module for [`MeloncraftResourceForwardingPlugin`].

use bevy::app::{App, Plugin, Update};

use crate::{difficulty, ticking, world_border, world_spawn};

/// Registers systems to send resources to clients in packets, when the resources change or players
/// join.
///
/// Add this plugin if you want to be able to just add and modify resources in your app and have
/// them automatically sent to clients without you needing to think about it.
pub struct MeloncraftResourceForwardingPlugin;

impl Plugin for MeloncraftResourceForwardingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, difficulty::send_difficulty_on_join);
        app.add_systems(Update, difficulty::send_difficulty_on_change);

        app.add_systems(Update, world_spawn::send_world_spawn_on_join);
        app.add_systems(Update, world_spawn::send_world_spawn_on_change);

        app.add_systems(Update, ticking::send_ticking_state_on_join);
        app.add_systems(Update, ticking::send_ticking_state_on_change);

        app.add_systems(Update, world_border::send_world_border_center_on_join);
        app.add_systems(Update, world_border::send_world_border_center_on_change);
        app.add_systems(Update, world_border::send_world_border_diameter_on_join);
        app.add_systems(Update, world_border::send_world_border_diameter_on_change);
    }
}

