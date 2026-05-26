//! Module for adding health components to newly-logged-in players.

use bevy::ecs::entity::Entity;
use bevy::ecs::query::Added;
use bevy::ecs::system::{Commands, Query};
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_entity::health::food::{FoodHealth, FoodSaturation};
use meloncraft_logger::tracelog;
use meloncraft_player::PlayerMarker;

/// Add a [`CurrentHealth`], [`FoodHealth`] and [`FoodSaturation`] component to players when they
/// log in.
pub fn insert_health(mut commands: Commands, player_q: Query<Entity, Added<PlayerMarker>>) {
    for player in player_q {
        let mut entity = commands.entity(player);
        entity.insert((CurrentHealth(20.0), FoodHealth(20), FoodSaturation(5.0)));
        tracelog!(
            "Added CurrentHealth, FoodHealth, FoodSaturation components to player {}",
            player
        );
    }
}
