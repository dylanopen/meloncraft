//! Module for [`WeatherIntensity`]-related world resources.

use bevy::ecs::resource::Resource;
use meloncraft_core::WeatherIntensity;

/// Component representing the intensity of the rain in the main world.
///
/// See [`WeatherIntensity`] for the format.
///
/// The `resource_forwarding` crate will automatically forward this into game events for
/// starting/stopping rain, as well as setting its intensity.
///
/// Note: Seems to change both sky color and lighting.
/// Rain level ranging from 0.0 to 1.0.
/// See <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Event>
#[derive(Resource, Debug, Clone)]
pub struct RainIntensity(pub WeatherIntensity);
