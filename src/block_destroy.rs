use valence::{BlockState, ChunkLayer, GameMode, action::{DiggingEvent, DiggingState}, app::{App, Plugin, Update}, prelude::{EventReader, Query}};

pub struct BlockDestroyPlugin;

impl Plugin for BlockDestroyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, block_destroy_handler);
    }
}

fn block_destroy_handler(
    clients: Query<&GameMode>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<DiggingEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let game_mode = clients.get(event.client).unwrap();

        if *game_mode == GameMode::Creative {
            layer.set_block(event.position, BlockState::AIR);
        }
        if *game_mode == GameMode::Survival && event.state == DiggingState::Stop {
            layer.set_block(event.position, BlockState::AIR);
        }
    }
}
