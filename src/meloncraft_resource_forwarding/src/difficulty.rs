//! Packet forwarders for [`Difficulty`] resource.

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::Added;
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundChangeDifficulty;
use meloncraft_player::{Difficulty, PlayerMarker};

pub fn send_difficulty_on_join(
    new_player_q: Query<Entity, Added<PlayerMarker>>,
    difficulty: Res<Difficulty>,
    mut change_difficulty_pw: MessageWriter<ClientboundChangeDifficulty>,
) {
    for client in new_player_q {
        change_difficulty_pw.write(ClientboundChangeDifficulty {
            client,
            difficulty: *difficulty,
            difficulty_locked: false,
        });
    }
}
