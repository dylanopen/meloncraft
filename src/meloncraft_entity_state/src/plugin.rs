//! Module for [`MeloncraftEntityStatePlugin`].

use bevy::app::{App, Plugin, Update};

use crate::location;

pub struct MeloncraftEntityStatePlugin;

/// Meloncraft plugin to handle entity events and store them as components on the Bevy Entity.
/// For example, this responds to `EntityMoved` messages by storing the `EntityPosition` component
/// on the entity, which can be queried later from the ECS.
impl Plugin for MeloncraftEntityStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, location::save_old_location);
    }
}
