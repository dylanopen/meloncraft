//! Module for struct component [`Locale`].

use bevy::prelude::Component;

/// The locale (region and language information) of a player.
/// This contains information about the player's region as well as their language preference.
///
/// *Component of a player entity*.
///
/// ## Packet references
/// - Sent in the `ServerboundClientInformation` packet, during configuration.
///
/// ## Format
/// The locale component has a string which is usually in the format of `language_REGION`, for
/// example, `en_CA` for *English*, *Canada*.
#[derive(Component, Debug, Clone)]
pub struct Locale(

    /// The string representing the player's [`Locale`].
    ///
    /// ## Format
    /// This field is usually in the format of `language_REGION`, for example, `en_CA` for
    /// *English*, *Canada*.
    pub String
);
