//! Packet forwarders for [`Difficulty`] resource.

use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Added, With};
use bevy::ecs::system::{Query, Res};
use meloncraft_packets::ClientboundChangeDifficulty;
use meloncraft_player::PlayerMarker;
use meloncraft_server_info::difficulty::Difficulty;

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

pub fn send_difficulty_on_change(
    player_q: Query<Entity, With<PlayerMarker>>,
    difficulty: Res<Difficulty>,
    mut change_difficulty_pw: MessageWriter<ClientboundChangeDifficulty>,
) {
    if !difficulty.is_changed() { return; }

    for client in player_q {
        change_difficulty_pw.write(ClientboundChangeDifficulty {
            client,
            difficulty: *difficulty,
            difficulty_locked: false,
        });
    }
}

