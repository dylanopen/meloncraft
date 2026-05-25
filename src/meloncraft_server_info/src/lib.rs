//! Crate containing data types and resources for the current state of the Meloncraft server, as
//! well as various configuration options (e.g. server MOTD, max players, etc.) as well as any other
//! data type related to the server's info.
//! For the [Meloncraft](https://github.com/dylanopen/meloncraft) server implementation.
//!
//! See the documentation for the individual modules, structs and enums for more information on the
//! resources contained in this crate, and their usage in packets and systems throughout the
//! codebase.
//!
//! ## Pure `type` crate
//! `meloncraft_server_info` is a *type* crate. This means it doesn't hold any systems or stateful
//! functions. It just holds datatypes and resources related to the server's current / configured
//! state, types which themselves are *used* by systems and functions in other crates.
//! Impure functions are not allowed in this crate, and so should any functions not directly related
//! to operating on the datatype itself only (utility functions for conversions, instantiation,
//! logic, etc. are fine but only if they operate on the datatype itself, and not on any external
//! state or external mutable parameters).

pub mod max_players;
pub mod motd;
pub mod icon;
pub mod online_players;
pub mod difficulty;
pub mod world_spawn;
pub mod tick;
pub mod world_border;
