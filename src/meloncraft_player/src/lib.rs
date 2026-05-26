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

mod plugin;
mod allow_player_listings;
mod chat_colors;
mod chat_mode;
mod displayed_skin_parts;
mod game_profile;
mod locale;
mod main_hand;
mod particle_rendering;
mod text_filtering;
mod username;
mod uuid;
mod view_distance;
mod action_status;
mod marker;

pub mod client_action;
pub mod ability;
pub mod experience;

pub use plugin::MeloncraftPlayerPlugin;
pub use allow_player_listings::AllowPlayerListings;
pub use chat_colors::ChatColors;
pub use chat_mode::ChatMode;
pub use displayed_skin_parts::DisplayedSkinParts;
pub use game_profile::GameProfile;
pub use game_profile::GameProfileProperties;
pub use locale::Locale;
pub use main_hand::MainHand;
pub use particle_rendering::ParticleRenderingMode;
pub use text_filtering::EnableTextFiltering;
pub use username::Username;
pub use uuid::Uuid;
pub use view_distance::ClientViewDistance;
pub use action_status::PlayerActionStatus;
pub use marker::PlayerMarker;
pub use ability::*;
