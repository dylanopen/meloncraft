#[derive(Debug, Clone)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl From<GameMode> for u8 {
    fn from(value: GameMode) -> Self {
        match value {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        }
    }
}

impl TryFrom<u8> for GameMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameMode::Survival),
            1 => Ok(GameMode::Creative),
            2 => Ok(GameMode::Adventure),
            3 => Ok(GameMode::Spectator),
            _ => Err(()),
        }
    }
}
