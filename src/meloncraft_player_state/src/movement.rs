use bevy::ecs::system::Commands;
use bevy::prelude::MessageReader;
use meloncraft_packets::ServerboundSetPlayerPosition;

pub fn save_new_location(
    mut set_position_pr: MessageReader<ServerboundSetPlayerPosition>,
    mut commands: Commands,
) {
    for packet in set_position_pr.read() {
        commands.entity(packet.client)
            .insert(packet.position.clone());
    }
}