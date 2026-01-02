use crate::status_request::respond_to_status_request;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{IntoScheduleConfigs, resource_exists};

pub mod max_players;
pub mod motd;
pub mod online_players;
mod status_request;

pub struct MeloncraftServerListPlugin;

impl Plugin for MeloncraftServerListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, respond_to_status_request);
    }
}
