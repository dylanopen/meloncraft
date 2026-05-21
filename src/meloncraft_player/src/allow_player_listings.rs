//! Module for component struct [`AllowPlayerListings`].

use bevy::prelude::Component;

/// Component storing whether the player has consented to being visible in player listings.
/// *Component for a player/client entity*.
///
/// - If true, the player **will** be shown in listings, e.g. when hoving over the player count
///   in the multiplayer menu, their name will be shown.
/// - If false, the server **should not** display their username in listings. You may want to show
///   'anonymous player' or something similar.
///
/// You technically do not have to acknowledge this request, but you *should*.
#[derive(Component, Clone, Debug, Copy, Eq, PartialEq)]
pub struct AllowPlayerListings(pub bool);
