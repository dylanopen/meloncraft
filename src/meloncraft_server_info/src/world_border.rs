//! Module for resources related to world borders.

use bevy::ecs::resource::Resource;
use bevy::math::DVec2;

/// The center block of the world, where the world border's radius will apply from.
/// This basically sets the 'middle block' of the border.
#[derive(Resource, Debug, Clone)]
pub struct WorldBorderCenter(pub DVec2);

/// The size of the world border, as a **diameter**.
/// A diameter of 100.0, if [`WorldBorderCenter`] is `0,0`, would stretch from `-50` to `+50` in
/// each direction.
#[derive(Resource, Debug, Clone)]
pub struct WorldBorderDiameter(pub f32);
