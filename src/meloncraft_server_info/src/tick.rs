//! Module for resource structs [`TickRate`] and [`TickingFrozen`].

use bevy::ecs::resource::Resource;

/// Stores the target tick rate of the server.
///
/// See [`TickRate::0`] for information about the contained float value.
///
/// I'm not sure if this should instead store the *actual* current TPS? It depends on what the
/// `ClientboundSetTickingState` packet wants
/// (see <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Set_Ticking_State>).
#[derive(Resource, Debug, Clone, Copy)]
pub struct TickRate(
    /// The target tick rate, in ticks per second (TPS), as an `f32`.
    /// Default: `20.0`, meaning 20 ticks occur each second (0.05 seconds per tick).
    pub f32,
);

/// Resource storing whether or not the server's ticking state is frozen.
///
/// See [`TickingFrozen::0`] for its field.
#[derive(Resource, Debug, Clone, Copy)]
pub struct TickingFrozen(
    /// Whether or not the server's ticking state is frozen.
    /// - `true` if the server's ticking is frozen.
    /// - `false` if it is not.
    pub bool,
);
