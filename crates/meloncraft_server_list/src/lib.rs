use bevy::app::{App, Plugin};

pub mod motd;
pub mod max_players;
pub mod online_players;

pub struct MeloncraftServerListPlugin;

impl Plugin for MeloncraftServerListPlugin {
    fn build(&self, app: &mut App) {

    }
}