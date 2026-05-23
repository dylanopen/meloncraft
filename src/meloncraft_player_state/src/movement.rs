use bevy::prelude::{MessageReader, MessageWriter, Query, With};
use meloncraft_entity::position::moved::EntityMoved;
use meloncraft_entity::position::EntityPosition;
use meloncraft_packets::ServerboundSetPlayerPosition;
use meloncraft_player::GameProfile;

/// System for forwarding player movement packets as [`EntityMoved`] messages.
///
/// This system listens for [`ServerboundSetPlayerPosition`] packets, retrieves the player's old
/// position from the ECS query, and writes an [`EntityMoved`] message with the old and
/// new positions.
///
/// If the player does not have an existing position (i.e., they are new or their position has not
/// been tracked yet), the system will use the new position from the packet as the old position in
/// the [`EntityMoved`] message (i.e. [`EntityMoved::new_position`] will equal [`EntityMoved::old_position`]).
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

