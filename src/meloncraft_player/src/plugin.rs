use bevy::app::{App, Plugin};

use crate::client_action::UpdateClientPlayerAction;

pub struct MeloncraftPlayerPlugin;

impl Plugin for MeloncraftPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UpdateClientPlayerAction>();
    }
}
