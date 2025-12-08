use valence::{LayerBundle, Server, app::{App, Plugin, Startup, Update}, client::despawn_disconnected_clients, ident, prelude::{BiomeRegistry, Commands, DimensionTypeRegistry, Res}};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, despawn_disconnected_clients);
    }
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
}

