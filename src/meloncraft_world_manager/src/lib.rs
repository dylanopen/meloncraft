pub mod marker;
mod sender;
mod request_forwarding;

use bevy::app::{App, Plugin, Startup, Update};
use meloncraft_world::world::World;
use crate::marker::Overworld;

pub struct MeloncraftWorldManagerPlugin;

impl Plugin for MeloncraftWorldManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sender::send_chunk);
        app.add_systems(Update, request_forwarding::send_requested_chunks);
        app.add_systems(Update, request_forwarding::send_generated_chunks);
    }
}
