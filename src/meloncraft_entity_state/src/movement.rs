//! Systems for managing state in response to location changes of entities.

use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query};
use meloncraft_entity::position::EntityPosition;
use meloncraft_entity::position::last::LastEntityPosition;
use meloncraft_entity::position::moved::EntityMoved;
use meloncraft_entity::position::teleport::TeleportEntity;

pub fn save_old_location(
    mut entity_moved_mr: MessageReader<EntityMoved>,
    mut commands: Commands,
) {
    for entity_moved in entity_moved_mr.read() {
        commands.entity(entity_moved.entity)
            .insert(LastEntityPosition(entity_moved.old_position.clone()));
    }
}

pub fn save_new_location(
    mut entity_moved_mr: MessageReader<EntityMoved>,
    mut commands: Commands,
) {
    for entity_moved in entity_moved_mr.read() {
        commands.entity(entity_moved.entity)
            .insert(entity_moved.new_position.clone());
    }
}

pub fn move_on_teleport(
    mut teleport_entity_mr: MessageReader<TeleportEntity>,
    entity_position_q: Query<&EntityPosition>,
    mut entity_moved_mw: MessageWriter<EntityMoved>,
) {
    for teleport_entity in teleport_entity_mr.read() {
        let old_position = entity_position_q.get(teleport_entity.entity).unwrap_or(&teleport_entity.new_position);
        entity_moved_mw.write(EntityMoved {
            entity: teleport_entity.entity,
            old_position: old_position.clone(),
            new_position: teleport_entity.new_position.clone(),
        });
    }
}

// TODO: send entity position update packets upon an `EntityMoved` message.

