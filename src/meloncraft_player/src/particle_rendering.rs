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
    All,
    Decreased,
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

