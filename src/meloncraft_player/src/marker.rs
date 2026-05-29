//! Module for component struct [`PlayerMarker`].

use bevy::ecs::component::Component;

/// Marker to allow querying specifically for player entities. This is added to a client once they
/// have *finished* spawning into the world, in the **play** connection state.
#[derive(Component)]
pub struct PlayerMarker;

/// Marker to allow querying specifically for player entities. This is added to a client once the
/// *library user* has added any components they need to add before they are acted upon (e.g. positions).
#[derive(Component)]
pub struct LoadedPlayer;
