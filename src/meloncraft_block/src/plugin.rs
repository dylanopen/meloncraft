use bevy::app::{App, Plugin};
use crate::broken::BlockBroken;

pub struct MeloncraftBlockPlugin;

impl Plugin for MeloncraftBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BlockBroken>();
    }
}