//! Module for enum component [`ParticleRenderingMode`].

use bevy::prelude::Component;

/// The player's choice of particles they want to render.
/// Depending on the user's configuration, clients may request to only be sent a minimal or 
/// decreased number of particles.
///
/// *This is a component for player entities*.
///
/// The server should use this component when sending particle packets to determine which particles
/// they should send to the client.
///
/// ## Protocol representation
/// See the variants of this enum for the protocol IDs (as `i32`s) of each variant.
#[derive(Component, Debug, Clone)]
pub enum ParticleRenderingMode {

    /// The client wants to receive all particles.
    /// **Protocol ID: `0`**.
    ///
    /// You should probably send all particles on the server (that are close enough to the player)
    /// to the client.
    All,

    /// The client wants to receive a decreased number of particles.
    /// **Protocol ID: `1`**.
    /// This means that they want fewer particles than in [`ParticleRenderingMode::All`], but more
    /// than in [`ParticleRenderingMode::Minimal`]. The 'middle-ground'.
    ///
    /// The exact number of particles you should send to the client is up to you, but it should be
    /// less than the number of particles you would send if the client had
    /// [`ParticleRenderingMode::All`], and more than the number of particles you would send if the
    /// client had [`ParticleRenderingMode::Minimal`].
    Decreased,

    /// The client wants to receive a minimal number of particles.
    /// **Protocol ID: `2`**.
    /// This means that they want fewer particles than in both [`ParticleRenderingMode::All`] and
    /// [`ParticleRenderingMode::Decreased`].
    ///
    /// You should send as few particles as possible to the client, whether this is by only sending
    /// a few particles, limiting it to the most important ones, or just not sending any particles
    /// at all is up to you.
    /// You *must* send fewer than the number of particles you would send if the client had either
    /// [`ParticleRenderingMode::All`] or [`ParticleRenderingMode::Decreased`].
    Minimal,
}

impl TryFrom<i32> for ParticleRenderingMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(Self::All),
            1 => Ok(Self::Decreased),
            2 => Ok(Self::Minimal),
            _ => Err(()),
        };
    }
}

impl From<ParticleRenderingMode> for i32 {
    fn from(value: ParticleRenderingMode) -> Self {
        return match value {
            ParticleRenderingMode::All => 0,
            ParticleRenderingMode::Decreased => 1,
            ParticleRenderingMode::Minimal => 2,
        }
    }
}

