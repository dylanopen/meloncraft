//! Module for [`MeloncraftEntityPlugin`].

use bevy::app::{App, Plugin};
use crate::position::moved::EntityMoved;

/// Meloncraft plugin to initialize messages related to entities.
///
/// ## Registered messages
/// Adding this plugin will register the following messagse:
/// - [`EntityMoved`]
pub struct MeloncraftEntityPlugin;

impl Plugin for MeloncraftEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EntityMoved>();
    }
}
