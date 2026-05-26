//! Module for player *ability* components.
//! See <https://github.com/dylanopen/meloncraft/issues/60>.

use bevy::ecs::component::Component;

/// This component is present on any player/entity who *cannot be hurt*.
/// Any player who can be damaged *will not have* this component.
/// 
/// You may want to query for `Without<Invulnerable>` in any systems which involve a player taking
/// damage, for example.
#[derive(Component, Debug, Clone, Copy)]
pub struct Invulnerable;

/// This component is present on any player who is *currently* flying.
/// Any player not flying will not have it on their entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct IsFlying;

