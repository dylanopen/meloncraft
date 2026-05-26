//! Module for resource struct [`GameTime`].

use bevy::ecs::resource::Resource;

/// The current Minecraft day time, in ticks.
///
/// This value is changed in many ways:
/// - Increases by 1 each tick (1/20th of a second) that passes.
/// - Increases when a player sleeps in a bed to skip the night.
/// - Can legally be changed manually, by changing the resource yourself or by defining a time set
///   command.
/// 
/// This determines the daylight cycle and can show in the client's F3 screen as the 'day count'.
#[derive(Resource, Debug, Clone, Copy)]
pub struct DayTime(pub u64);

