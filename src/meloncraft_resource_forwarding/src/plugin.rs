//! Module for [`MeloncraftResourceForwardingPlugin`].

use bevy::app::{App, Plugin, Update};

use crate::{data, difficulty, ticking, time, weather_intensity, world_border, world_spawn};

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
        app.add_systems(
            Update,
            world_border::send_world_border_warning_delay_on_join,
        );
        app.add_systems(
            Update,
            world_border::send_world_border_warning_delay_on_change,
        );
        app.add_systems(
            Update,
            world_border::send_world_border_warning_distance_on_join,
        );
        app.add_systems(
            Update,
            world_border::send_world_border_warning_distance_on_change,
        );

        app.add_systems(Update, time::send_set_time_on_join);
        app.add_systems(Update, time::send_set_time_on_change);

        app.add_systems(Update, data::send_server_data_on_join);
        app.add_systems(Update, data::send_server_data_on_change_motd);

        app.add_systems(Update, weather_intensity::send_rain_on_join);
        app.add_systems(Update, weather_intensity::send_rain_on_change);
        app.add_systems(Update, weather_intensity::send_thunder_on_join);
        //app.add_systems(Update, weather_intensity::send_thunder_on_change);
    }
}
