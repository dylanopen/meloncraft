//! Module for [`MeloncraftResourceForwardingPlugin`].

use bevy::app::{App, Plugin};

/// Registers systems to send resources to clients in packets, when the resources change or players
/// join.
///
/// Add this plugin if you want to be able to just add and modify resources in your app and have
/// them automatically sent to clients without you needing to think about it.
pub struct MeloncraftResourceForwardingPlugin;

impl Plugin for MeloncraftResourceForwardingPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

