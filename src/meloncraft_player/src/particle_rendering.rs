use bevy::prelude::Component;

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

