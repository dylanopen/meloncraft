//! Module for [`MeloncraftBlockBreakingPlugin`].

use bevy::app::{App, Plugin, Update};

use crate::forward;

pub struct MeloncraftBlockBreakingPlugin;

impl Plugin for MeloncraftBlockBreakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, forward::set_broken_blocks);
    }
}

