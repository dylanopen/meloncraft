//! Systems for managing state in response to location changes of entities.

use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query};
use bevy::math::{DVec3, IVec3};
use meloncraft_entity::position::EntityPosition;
use meloncraft_entity::position::current_chunk::CurrentChunk;
use meloncraft_entity::position::last::LastEntityPosition;
use meloncraft_entity::position::moved::EntityMoved;
use meloncraft_entity::position::teleport::TeleportEntity;

pub fn save_old_location(mut entity_moved_mr: MessageReader<EntityMoved>, mut commands: Commands) {
    for entity_moved in entity_moved_mr.read() {
        commands
            .entity(entity_moved.entity)
            .insert(LastEntityPosition(entity_moved.old_position.clone()));
    }
}

pub fn save_new_location(mut entity_moved_mr: MessageReader<EntityMoved>, mut commands: Commands) {
    for entity_moved in entity_moved_mr.read() {
        commands
            .entity(entity_moved.entity)
            .insert(entity_moved.new_position.clone());
    }
}

pub fn move_on_teleport(
    mut teleport_entity_mr: MessageReader<TeleportEntity>,
    entity_position_q: Query<&EntityPosition>,
    mut entity_moved_mw: MessageWriter<EntityMoved>,
) {
    for teleport_entity in teleport_entity_mr.read() {
        let old_position = entity_position_q
            .get(teleport_entity.entity)
            .unwrap_or(&teleport_entity.new_position);
        entity_moved_mw.write(EntityMoved {
            entity: teleport_entity.entity,
            old_position: old_position.clone(),
            new_position: teleport_entity.new_position.clone(),
        });
    }
}

pub fn edit_current_chunk(
    mut entity_moved_mr: MessageReader<EntityMoved>,
    mut current_chunk_q: Query<&mut CurrentChunk>,
) {
    const fn dvec3_to_ivec3(dvec3: DVec3) -> IVec3 {
        #[expect(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            reason = "Required to convert a f64 to an i32"
        )]
        return IVec3::new(
            dvec3.x.floor() as i32,
            dvec3.y.floor() as i32,
            dvec3.z.floor() as i32,
        );
    }

    for entity_moved in entity_moved_mr.read() {
        let old_chunk = dvec3_to_ivec3(entity_moved.old_position.location) / 16;
        let new_chunk = dvec3_to_ivec3(entity_moved.new_position.location) / 16;
        if old_chunk == new_chunk {
            continue;
        }
        current_chunk_q
            .get_mut(entity_moved.entity)
            .unwrap()
            .location = new_chunk;
    }
}

// TODO: send entity position update packets upon an `EntityMoved` message.
