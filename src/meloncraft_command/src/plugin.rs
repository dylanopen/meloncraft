//! Module for [`MeloncraftCommandPlugin`].

use bevy::app::{App, Plugin};

use crate::raw::RawCommand;

/// Plugin to register messages related to command processing.
///
/// ## Registered messages:
/// - [`RawCommand`]
pub struct MeloncraftCommandPlugin;

impl Plugin for MeloncraftCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RawCommand>();
    }
}
