//! Module for [`MeloncraftLoggerPlugin`].

use bevy::app::{App, Plugin, PreUpdate};
use bevy::ecs::message::MessageWriter;

use crate::buffer::LOG_BUFFER;
use crate::log::Log;

/// Plugin to register logging events and forward buffered logs to the ECS.
///
/// ## Registered messages
/// - [`Log`]
///
/// ## Forwarding
/// - When logs are added to the [`LOG_BUFFER`], e.g. when using a macro like `warn!`, this plugin
///   will write the [`Log`] as a Bevy message, allowing you to listen for them in the ECS.
pub struct MeloncraftLoggerPlugin;

impl Plugin for MeloncraftLoggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Log>();
        app.add_systems(PreUpdate, forward_log_buffer);
    }
}

/// System to write the logs in the [`LOG_BUFFER`] as Bevy [`Log`] messages.
fn forward_log_buffer(mut log_mw: MessageWriter<Log>) {
    let mut buffer = LOG_BUFFER.lock().unwrap();
    log_mw.write_batch(buffer.clone());
    // TODO: how can we take this value and clear the buffer *without* any clones? Not sure.
    buffer.clear();
}
