//! Module for resources related to world borders.

use bevy::ecs::resource::Resource;
use bevy::math::DVec2;

/// The center block of the world, where the world border's radius will apply from.
/// This basically sets the 'middle block' of the border.
#[derive(Resource, Debug, Clone)]
pub struct WorldBorderCenter(pub DVec2);

