//! Module for component struct [`LastEntityPosition`].

use crate::position::EntityPosition;
use bevy::prelude::Component;

/// Convenience component for storing the *last* position an entity was in.
///
/// This should store their previous position, either from the last tick or the last
/// position-related packet or entity update, depending on the use case.
///
/// This is useful for systems that need to compare an entity's current position to
/// their previous position.
///
/// Not automatically updated by internal systems: you need to update this component yourself if you
/// need it.
///
/// ## Fields
/// - [`LastEntityPosition::0`]: the [`EntityPosition`] of the entity's last position. See
///   [`EntityPosition`] for information about the data in this field.
#[derive(Debug, Clone, Component)]
pub struct LastEntityPosition(pub EntityPosition);
