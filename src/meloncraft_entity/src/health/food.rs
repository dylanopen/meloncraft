//! Module for component struct [`FoodHealth`].

use bevy::ecs::component::Component;

/// The number of half-hunger bars the entity has.
/// For a player, this ranges from 0 (starving) to 20 (completely full).
#[derive(Component, Debug, Clone, Copy)]
pub struct FoodHealth(pub i32);
