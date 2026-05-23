//! Module for [`MeloncraftDefaultCommandsPlugin`].

use bevy::app::{App, Plugin, Update};

use crate::teleport;

/// Plugin to register the default commands.
/// This currently registers the following commands:
/// - [`teleport`]
pub struct MeloncraftDefaultCommandsPlugin;

impl Plugin for MeloncraftDefaultCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, teleport::cmd_teleport);
    }
}
