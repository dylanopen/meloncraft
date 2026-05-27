//! Module for component struct [`BossbarHealth`].

use bevy::ecs::component::Component;

/// Component to add to a player who has a bossbar.
///
/// The fraction of the bossbar which should be filled.
/// `0.0` means the bar is completely empty, `1.0` means it is completely full.
#[derive(Component, Debug, Clone, Copy)]
pub struct BossbarHealth(pub f32);
