//! Module for component struct [`BossbarMarker`].

use bevy::ecs::component::Component;

/// Component to add to any entity which represents a *bossbar*.
///
/// If added to an entity, the entity should also have the other components in a
/// [`BossbarBundle`][`crate::bundle::BossbarBundle`].
///
/// You can query for entities `With` this marker if you want to get specific bossbar components.
///
/// See [`ActiveBossbars`][`crate::active::ActiveBossbars`] for how to show the bossbar to a specific
/// player.
#[derive(Component, Debug, Clone, Copy)]
pub struct BossbarMarker;
