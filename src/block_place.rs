use tracing::{error, info};
use valence::{BlockState, ChunkLayer, Direction, GameMode, Hand, action::{DiggingEvent, DiggingState}, app::{App, Plugin, Update}, block::{BlockKind, PropName, PropValue}, interact_block::InteractBlockEvent, inventory::HeldItem, prelude::{EventReader, Inventory, Query}};

pub struct BlockPlacePlugin;

impl Plugin for BlockPlacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, block_place_handler);
    }
}

fn block_place_handler(
    mut clients: Query<(&GameMode, &mut Inventory, &HeldItem)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((game_mode, mut inventory, held_item)) = clients.get_mut(event.client) else {
            error!("Failed to get client '{}' data for block placement", event.client);
            continue;
        };
        if event.hand != Hand::Main {
            info!("Only main-hand block placement is currently supported");
            continue;
        }
        let hand_slot_id = held_item.slot();
        let item_stack = inventory.slot(hand_slot_id);
        if item_stack.is_empty() { continue; }

        let Some(block_kind) = BlockKind::from_item_kind(item_stack.item) else { continue };
        let new_block_pos = event.position.get_in_direction(event.face);
        let state = block_kind.to_state().set(
            PropName::Axis,
            match event.face {
                Direction::Down | Direction::Up => PropValue::Y,
                Direction::North | Direction::South => PropValue::Z,
                Direction::West | Direction::East => PropValue::X,
            },
        );
        layer.set_block(new_block_pos, state);
    }
}

