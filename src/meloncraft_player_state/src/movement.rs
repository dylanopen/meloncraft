use bevy::ecs::system::Commands;
use bevy::prelude::{MessageReader, MessageWriter, Query, With};
use meloncraft_entity::position::moved::EntityMoved;
use meloncraft_entity::position::EntityPosition;
use meloncraft_entity::position::last::LastEntityPosition;
use meloncraft_packets::ServerboundSetPlayerPosition;
use meloncraft_player::GameProfile;

pub fn fwd_player_moved(
    mut set_position_pr: MessageReader<ServerboundSetPlayerPosition>,
    mut player_moved_mw: MessageWriter<EntityMoved>,
    existing_positions: Query<&EntityPosition, With<GameProfile>>,
) {
    for packet in set_position_pr.read() {
        let old_position = existing_positions.get(packet.client)
            .map_or_else(|_| return packet.position.clone(), |old_position| return old_position.clone());
        player_moved_mw.write(EntityMoved {
            entity: packet.client,
            old_position,
            new_position: packet.position.clone(),
        });
    }
}

pub fn save_new_location(
    mut player_moved_mr: MessageReader<EntityMoved>,
    mut commands: Commands,
) {
    for player_moved in player_moved_mr.read() {
        commands.entity(player_moved.entity)
            .insert(player_moved.new_position.clone());
    }
}

pub fn save_old_location(
    mut player_moved_mr: MessageReader<EntityMoved>,
    mut commands: Commands,
) {
    for player_moved in player_moved_mr.read() {
        commands.entity(player_moved.entity)
            .insert(LastEntityPosition(player_moved.old_position.clone()));
    }
}
