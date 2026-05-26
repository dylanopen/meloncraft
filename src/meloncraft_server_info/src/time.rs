//! Module for resource struct [`GameTime`].

use bevy::ecs::message::Message;
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

/// The amount of time (in ticks) that the server has been online for, **total**.
/// This does *not* change when sleeping, setting the time, or any other 'artificial' ways of
/// passing time.
///
/// Also known as `World Age`.
/// Dividing [`OpenTime::0`] by `20` will give the total runtime, in seconds, of the server.
#[derive(Resource, Debug, Clone, Copy)]
pub struct OpenTime(pub u64);

/// Whether or not the server should advance time each tick.
/// - True means that [`DayTime`] and [`OpenTime`] will increase by `1` for every server tick.
/// - False means that [`DayTime`] and [`OpenTime`] are frozen unless manually changed.
#[derive(Resource, Debug, Clone, Copy)]
pub struct DaylightCycle(pub bool);

/// Message to send when you want to forward the current game time resources to all clients.
///
/// You should send this after manually updating the time, as the forwarding system wouldn't be able
/// to distinguish between manual component changes and automatic increments each tick.
#[derive(Message, Debug, Clone)]
pub struct TimeChanged;
