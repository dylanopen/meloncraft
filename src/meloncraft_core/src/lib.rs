//! Crate containing core datatypes used in various places throughout
//! [Meloncraft](https://github.com/dylanopen/meloncraft).
//! 
//! The main purpose of this crate is to store datatypes which don't fit into any other datatype
//! crates, and aren't complex enough to warrant their own crate.
//! For example, the `Difficulty` enum is used in many places throughout the codebase, but it's
//! nothing more than a simple enum, so doesn't need its own crate.
//!
//! ## Pure `type` crate
//! `meloncraft_core` is a *type* crate. This means it doesn't hold any systems or stateful
//! functions. It just holds datatypes which themselves are *used* by systems and functions in other
//! crates.
//! Impure functions are not allowed in this crate, and so should any functions not directly related
//! to operating on the datatype itself only (utility functions for conversions, instantiation,
//! logic, etc. are fine but only if they operate on the datatype itself, and not on any external
//! state or external mutable parameters).

pub mod datapack;
pub mod disconnect_report;
pub mod identifier;
pub mod pause_menu;
pub mod resource_pack_load_result;
pub mod gamemode;
pub mod demo_event;
pub mod weather_intensity;
pub mod game_event;

pub use disconnect_report::DisconnectReport;
pub use identifier::Identifier;
pub use resource_pack_load_result::ResourcePackLoadResult;
pub use gamemode::GameMode;
pub use demo_event::DemoEventType;
pub use weather_intensity::WeatherIntensity;
