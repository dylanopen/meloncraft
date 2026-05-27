//! Module for various component flags on a bossbar.

use bevy::ecs::component::Component;

/// Component on a bossbar where:
/// - `true` means that the sky should be darkened while the bossbar is active.
/// - `false` means that the sky should **not** be darkened while the bossbar is active.
#[derive(Component, Debug, Clone)]
pub struct BossbarDarkensSky(pub bool);

/// Component on a bossbar where:
/// - `true` means that the bar is a dragon bar (should play music after health <= 0)
/// - `false` means that the bar is **not** a dragon bar (no music).
#[derive(Component, Debug, Clone)]
pub struct BossbarIsDragon(pub bool);

/// Component on a bossbar where:
/// - `true` means that the bar should create fog for the client.
/// - `false` means that the bar should **not** create fog for the client.
#[derive(Component, Debug, Clone)]
pub struct BossbarCreatesFog(pub bool);
