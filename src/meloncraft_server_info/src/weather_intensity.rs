//! Module for [`WeatherIntensity`]-related world resources.

use bevy::ecs::resource::Resource;
use meloncraft_core::WeatherIntensity;

/// Component representing the intensity of the rain in the main world.
/// Can be `0.0` for no rain.
///
/// See [`WeatherIntensity`] for the format.
///
/// The `resource_forwarding` crate will automatically forward this into game events for
/// starting/stopping rain, as well as setting its intensity.
///
/// Note: Seems to change both sky color and lighting.
/// Rain level ranging from 0.0 to 1.0.
/// See <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Event>.
#[derive(Resource, Debug, Clone)]
pub struct RainIntensity(pub WeatherIntensity);

/// Component representing the intensity of the thunder in the main world.
/// Can be `0.0` for no thunder.
///
/// See [`WeatherIntensity`] for the format.
///
/// The `resource_forwarding` crate will automatically forward this into game events for
/// starting/stopping thunder, as well as setting its intensity.
///
/// Note: Seems to change both sky color and lighting (same as Rain level change, but doesn't start
/// rain). It also requires rain to render by vanilla client.
/// Thunder level ranging from 0 to 1.
/// See <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Game_Event>.
#[derive(Resource, Debug, Clone)]
pub struct ThunderIntensity(pub WeatherIntensity);
