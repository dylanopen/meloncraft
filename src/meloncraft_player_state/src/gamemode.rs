use bevy::ecs::entity::Entity;
use bevy::ecs::query::Added;
use bevy::ecs::system::{Commands, Query};
use meloncraft_core::GameMode;
use meloncraft_player::PlayerMarker;

pub fn insert_gamemode(
    mut commands: Commands,
    added_player_q: Query<Entity, Added<PlayerMarker>>,
) {
    for added_player in added_player_q {
        commands.get_entity(added_player).unwrap().insert(GameMode::Survival);
    }
}
