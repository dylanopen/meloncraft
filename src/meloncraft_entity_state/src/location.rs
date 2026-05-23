//! Systems for managing state in response to location changes of entities.

use bevy::ecs::message::MessageReader;
use bevy::ecs::system::Commands;
use meloncraft_entity::position::last::LastEntityPosition;
use meloncraft_entity::position::moved::EntityMoved;

pub fn save_old_location(
    mut entity_moved_mr: MessageReader<EntityMoved>,
    mut commands: Commands,
) {
    for entity_moved in entity_moved_mr.read() {
        commands.entity(entity_moved.entity)
            .insert(LastEntityPosition(entity_moved.old_position.clone()));
    }
}
