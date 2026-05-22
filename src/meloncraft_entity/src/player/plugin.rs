//! Module for [`MeloncraftPlayerEntityPlugin`].

use bevy::app::{App, Plugin};
use crate::player::moved::PlayerMoved;

/// Meloncraft plugin to initialize messages related to player entities.
///
/// ## Messages
/// Adding this plugin will register the following messages:
/// - [`PlayerMoved`]
pub struct MeloncraftPlayerEntityPlugin;

impl Plugin for MeloncraftPlayerEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerMoved>();
    }
}
