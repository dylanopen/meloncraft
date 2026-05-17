use crate::ping_request::respond_to_ping_request;
use crate::status_request::respond_to_status_request;
use bevy::app::{App, Plugin, Update};

mod ping_request;
mod status_request;

pub struct MeloncraftServerListPlugin;

impl Plugin for MeloncraftServerListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, respond_to_status_request);
        app.add_systems(Update, respond_to_ping_request);
    }
}
