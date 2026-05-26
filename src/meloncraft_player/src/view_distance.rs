//! Module for struct component [`ClientViewDistance`].

use bevy::prelude::Component;

/// Stores the client's render distance request, as a `u8` number of chunks.
///
/// *Component for player entities*.
/// You should query for this component when, for example, writing `SendChunk` messages for queuing
/// chunk sends to a client.
///
/// This is a **radius** around the player.
/// For example, if a player chooses a render distance of `8`, they are requesting to receive chunks
/// in a `17x17` area around them (the chunk they are standing in, and 8 chunks in each direction).
/// The number of chunks sent in each direction is always an odd number (17 above), so that there is
/// always a central chunk to base the render distance around.
///
/// The server **does not** have to respect this: it can send fewer chunks than the client requests.
/// But it should not send *more* chunks than requested.
#[derive(Component, Debug, Clone, Copy)]
pub struct ClientViewDistance(
    /// The view distance value, as a radius of chunks around the player.
    pub u8,
);
