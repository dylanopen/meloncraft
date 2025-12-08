use std::fs;

use tracing::error;
use valence::{LayerBundle, Server, anvil::{AnvilLevel}, app::{Plugin, Startup}, ident, prelude::{BiomeRegistry, Commands, DimensionTypeRegistry, Res}};

pub struct WorldLoadPlugin;

impl Plugin for WorldLoadPlugin {
    fn build(&self, app: &mut valence::app::App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    server: Res<Server>,
) {
    let layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);
    if !fs::exists("world").unwrap() {
        error!("No 'world' folder found. A limbo server will be created instead (with no world).");
    }
    let level = AnvilLevel::new("world", &biomes);
    commands.spawn((layer, level));
}
