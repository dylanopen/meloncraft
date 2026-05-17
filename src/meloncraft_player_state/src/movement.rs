use bevy::ecs::system::Commands;
use bevy::prelude::{MessageReader, MessageWriter};
use meloncraft_entity::player::moved::PlayerMoved;
use meloncraft_packets::ServerboundSetPlayerPosition;

pub fn save_new_location(
    mut player_moved_mr: MessageReader<PlayerMoved>,
    mut commands: Commands,
) {
    for player_moved in player_moved_mr.read() {
        commands.entity(player_moved.entity)
            .insert(player_moved.new_position.clone());
    }
}

pub fn fwd_player_moved(
    mut set_position_pr: MessageReader<ServerboundSetPlayerPosition>,
    mut player_moved_mw: MessageWriter<PlayerMoved>,
) {
    for packet in set_position_pr.read() {
        player_moved_mw.write(PlayerMoved {
            entity: packet.client,
            old_position: packet.position.clone(), // For now, just set the old position to the new position, but this will cause future bugs so will soon be replaced.
            new_position: packet.position.clone(),
        });
    }
}