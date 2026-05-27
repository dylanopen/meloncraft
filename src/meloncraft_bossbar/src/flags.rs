//! Module for various component flags on a bossbar.

use bevy::ecs::component::Component;

/// Component on a bossbar where:
/// - `true` means that the sky should be darkened while the bossbar is active.
/// - `false` means that the sky should **not** be darkened while the bossbar is active.
#[derive(Component, Debug, Clone)]
pub struct BossbarDarkensSky(pub bool);
