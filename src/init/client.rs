use valence::{ChunkLayer, EntityLayer, GameMode, app::{App, Plugin, Update}, client::{Client, VisibleChunkLayer, VisibleEntityLayers}, entity::{EntityLayerId, Position}, prelude::{Added, Entity, Query, With}};

pub struct ClientInitializationPlugin;

impl Plugin for ClientInitializationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, initialize_joining_clients);
    }
}

/// This system will iterate for all newly joined clients that tick.
/// It sets up:
/// - their position
/// - their game mode
/// - their visible chunk layers
/// - their visible entity layers
fn initialize_joining_clients(
    mut clients: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layers,
        mut visible_entity_layers,
        mut position,
        mut game_mode,
    ) in &mut clients
    {
        let layer = layers.single();
        layer_id.0 = layer;
        visible_chunk_layers.0 = layer;
        visible_entity_layers.0.insert(layer);
        position.set([0.0, 100.0, 0.0]);
        *game_mode = GameMode::Creative;
    }
}

