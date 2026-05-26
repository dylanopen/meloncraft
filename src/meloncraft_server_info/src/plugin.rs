use bevy::app::{App, Plugin};

use crate::time::TimeChanged;

pub struct MeloncraftServerInfoPlugin;

impl Plugin for MeloncraftServerInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TimeChanged>();
    }
}
