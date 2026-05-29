//! Packet forwarders for [`Difficulty`] resource.

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, Changed, With};
use bevy::ecs::system::Query;
use meloncraft_entity::position::EntityPosition;
use meloncraft_packets::ClientboundChangeDifficulty;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::difficulty::Difficulty;

pub fn send_difficulty_on_join(
    new_player_q: Query<(Entity, &EntityPosition), Added<PlayerMarker>>,
    difficulty: Query<&Difficulty>,
    mut change_difficulty_pw: MessageWriter<ClientboundChangeDifficulty>,
) {
    for (client, player_position) in new_player_q {
        let difficulty = difficulty.get(player_position.world).unwrap();
        change_difficulty_pw.write(ClientboundChangeDifficulty {
            client,
            difficulty: *difficulty,
            difficulty_locked: false,
        });
    }
}

pub fn send_difficulty_on_change(
    player_q: Query<(Entity, &EntityPosition), With<PlayerMarker>>,
    difficulty_q: Query<(Entity, &Difficulty), Changed<Difficulty>>,
    mut change_difficulty_pw: MessageWriter<ClientboundChangeDifficulty>,
) {
    for (world, difficulty) in difficulty_q {
        for (client, player_position) in player_q {
            if player_position.world != world {
                continue;
            }
            change_difficulty_pw.write(ClientboundChangeDifficulty {
                client,
                difficulty: *difficulty,
                difficulty_locked: false,
            });
        }
    }
}
