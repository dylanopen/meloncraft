use bevy::app::{App, Plugin};
use crate::broken::{BlockBroken, PlayerBrokeBlock};

pub struct MeloncraftBlockPlugin;

impl Plugin for MeloncraftBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BlockBroken>();
        app.add_message::<PlayerBrokeBlock>();
    }
}