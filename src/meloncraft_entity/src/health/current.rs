//! Module for resource struct [`CurrentHealth`].

use bevy::ecs::component::Component;

/// The current health of an entity.
/// Usually a number between 0 (dead) and 20 (max) for players, but the range can depend on the
/// maximum health for target entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct CurrentHealth(pub f32);
