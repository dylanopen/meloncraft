//! Module for player *ability* components.
//! See <https://github.com/dylanopen/meloncraft/issues/60>.

use bevy::ecs::component::Component;

/// This component is tru on any player/entity who *cannot be hurt*.
/// False on any player who can be damaged.
///
/// You may want to query for `Without<Invulnerable>` in any systems which involve a player taking
/// damage, for example.
#[derive(Component, Debug, Clone, Copy)]
pub struct Invulnerable(pub bool);

/// This component is true on any player who is *currently* flying.
/// False on any player not flying.
#[derive(Component, Debug, Clone, Copy)]
pub struct IsFlying(pub bool);

/// This component is true on any player who is *allowed* to fly.
/// Any player without the ability to fly (by double pressing space) will have it as false.
#[derive(Component, Debug, Clone, Copy)]
pub struct CanFly(pub bool);

/// This component is true on any player who is able to *break any block instantly* (like in
/// creative mode).
/// Any player without this breaking ability will have this component be false.
#[derive(Component, Debug, Clone, Copy)]
pub struct CanInstantBreak(pub bool);

/// The speed, in arbitrary units as a float, that the player is able to travel at while they are
/// flying.
///
/// Default value: `0.05` for a normal creative-mode flying player.
#[derive(Component, Debug, Clone, Copy)]
pub struct FlySpeed(pub f32);

/// The distortion to the player's FOV. In vanilla, this can be changed with things like a speed
/// potion.
///
/// Default is `0.1` for a player without speed.
#[derive(Component, Debug, Clone, Copy)]
pub struct FovModifier(pub f32);
