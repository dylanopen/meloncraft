//! Module for component struct [`ActiveBossbars`].

use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;

/// Component to add to a *player* to indicate a list of bossbars they should be shown.
/// 
/// It is a list of references to **bossbar entities**: so each referenced entity must have the
/// components in a [`BossbarBundle`](`crate::bundle::BossbarBundle`].
///
/// This component should always be on a player, even if they are showing no bossbars: just clear
/// the internal array and the `component_forwarding` crate will do the packet sending for you.
#[derive(Component, Debug, Clone)]
pub struct ActiveBossbars(pub Vec<Entity>);
