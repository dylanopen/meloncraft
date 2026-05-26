//! Forwarding for health-related components when it is added/changed on a player.

use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Changed, Or};
use bevy::ecs::system::Query;
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_entity::health::food::{FoodHealth, FoodSaturation};
use meloncraft_packets::ClientboundSetHealth;

type HealthChanged = Or<(
    Changed<CurrentHealth>,
    Changed<FoodHealth>,
    Changed<FoodSaturation>,
)>;

pub fn send_health(
    player_q: Query<(Entity, &CurrentHealth, &FoodHealth, &FoodSaturation), HealthChanged>,
    mut set_health_pw: MessageWriter<ClientboundSetHealth>,
) {
    for (entity, current_health, food_health, food_saturation) in player_q {
        set_health_pw.write(ClientboundSetHealth {
            client: entity,
            current: *current_health,
            food: *food_health,
            saturation: *food_saturation,
        });
    }
}
