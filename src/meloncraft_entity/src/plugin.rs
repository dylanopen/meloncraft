//! Module for [`MeloncraftEntityPlugin`].

use bevy::app::{App, Plugin};
use crate::player::plugin::MeloncraftPlayerEntityPlugin;

/// Meloncraft plugin to initialize messages related to entities, as well as adding sub-plugins for
/// specific types of entities.
///
/// ## Sub-plugins
/// Adding this plugin will register the following other plugins:
/// - [`MeloncraftPlayerEntityPlugin`]
pub struct MeloncraftEntityPlugin;

impl Plugin for MeloncraftEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeloncraftPlayerEntityPlugin);
    }
}
