pub enum ParticleRenderingMode {
    All = 0,
    Decreased = 1,
    Minimal = 2,
}

impl TryFrom<i32> for ParticleRenderingMode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::All),
            1 => Ok(Self::Decreased),
            2 => Ok(Self::Minimal),
            _ => Err(()),
        }
    }
}
