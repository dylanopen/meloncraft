//! Module for component structs [`FoodHealth`] and [`FoodSaturation`].

use bevy::ecs::component::Component;

/// The number of half-hunger bars the entity has.
/// For a player, this ranges from 0 (starving) to 20 (completely full).
#[derive(Component, Debug, Clone, Copy)]
pub struct FoodHealth(pub i32);

/// The saturation of the entity.
///
/// Food saturation acts as a food “overcharge”. Food values will not decrease while the
/// saturation is over zero. New players logging in or respawning automatically get a
/// saturation of 5.0. Eating food increases the saturation as well as the food bar.
///
/// Seems to vary from 0.0 to 5.0 in integer increments.
///
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Health>.
#[derive(Component, Debug, Clone, Copy)]
pub struct FoodSaturation(pub f32);
