#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl From<GameMode> for u8 {
    fn from(value: GameMode) -> Self {
        return match value {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        };
    }
}

impl TryFrom<u8> for GameMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(GameMode::Survival),
            1 => Ok(GameMode::Creative),
            2 => Ok(GameMode::Adventure),
            3 => Ok(GameMode::Spectator),
            _ => Err(()),
        };
    }
}

impl From<GameMode> for f32 {
    fn from(value: GameMode) -> Self {
        return match value {
            GameMode::Survival => 0.0,
            GameMode::Creative => 1.0,
            GameMode::Adventure => 2.0,
            GameMode::Spectator => 3.0,
        };
    }
}

impl TryFrom<f32> for GameMode {
    type Error = ();
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        return match value {
            0.0 => Ok(GameMode::Survival),
            1.0 => Ok(GameMode::Creative),
            2.0 => Ok(GameMode::Adventure),
            3.0 => Ok(GameMode::Spectator),
            _ => Err(()),
        };
    }
}
