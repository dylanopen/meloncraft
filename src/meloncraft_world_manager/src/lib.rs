mod generator;
pub mod marker;
mod request_forwarding;
mod sender;
mod setblock;
mod state;

use bevy::app::{App, Plugin, Update};

pub struct MeloncraftWorldManagerPlugin;

impl Plugin for MeloncraftWorldManagerPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(PostStartup, generator::generate_empty_overworld);

        app.add_systems(Update, sender::send_chunk);
        app.add_systems(Update, request_forwarding::send_requested_chunks);
        app.add_systems(Update, request_forwarding::send_generated_chunks);
        app.add_systems(Update, state::store_generated_chunks);
        app.add_systems(Update, setblock::store_set_blocks);
        app.add_systems(Update, setblock::send_set_blocks);
    }
}
