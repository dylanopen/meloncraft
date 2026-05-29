//! Crate containing data types for representing ***player*** entity components, resources and
//! messages related to their state and events, for the
//! [Meloncraft](https://github.com/dylanopen/meloncraft) server implementation.
//!
//! See the documentation for the individual modules, structs and enums for more information on the
//! data types contained in this crate, and their usage in packets and systems throughout the
//! codebase.
//!
//! ## Pure `type` crate
//! `meloncraft_player` is a *type* crate. This means it doesn't hold any systems or stateful
//! functions. It just holds datatypes (and potentially message declarations & initialization)
//! which themselves are *used* by systems and functions in other crates.
//! Impure functions are not allowed in this crate, and so should any functions not directly related
//! to operating on the datatype itself only (utility functions for conversions, instantiation,
//! logic, etc. are fine but only if they operate on the datatype itself, and not on any external
//! state or external mutable parameters).

pub mod ability;
pub mod action_status;
pub mod allow_player_listings;
pub mod bundle;
pub mod chat_colors;
pub mod chat_mode;
pub mod client_action;
pub mod displayed_skin_parts;
pub mod experience;
pub mod game_profile;
pub mod locale;
pub mod main_hand;
pub mod marker;
pub mod particle_rendering;
pub mod plugin;
pub mod text_filtering;
pub mod username;
pub mod view_distance;

pub use ability::*;
pub use action_status::PlayerActionStatus;
pub use allow_player_listings::AllowPlayerListings;
pub use bundle::LoadedPlayerBundle;
pub use chat_colors::ChatColors;
pub use chat_mode::ChatMode;
pub use displayed_skin_parts::DisplayedSkinParts;
pub use game_profile::GameProfile;
pub use game_profile::GameProfileProperties;
pub use locale::Locale;
pub use main_hand::MainHand;
pub use marker::PlayerMarker;
pub use particle_rendering::ParticleRenderingMode;
pub use plugin::MeloncraftPlayerPlugin;
pub use text_filtering::EnableTextFiltering;
pub use username::Username;
pub use view_distance::ClientViewDistance;
