//! Module for enum [`GameMode`].

/// The game mode of a player.
/// This determines how the player interacts with the world, entities, etc.
/// See <https://minecraft.wiki/w/Game_mode> for more information.
///
/// ## Packet usage
/// - Most packets that use the [`GameMode`] enum internally convert the [`GameMode`] to a `u8` (or
///   from a `u8` to a [`GameMode`]) before sending (or after receiving) the packet.
/// - Some packets, such as the `ClientboundGameEvent` packet, represent it as an `f32`.
/// - `From` is implemented bidirectionally for both `u8` and `f32`.
/// - See the individual variants to find the number associated with each game mode.
#[derive(Debug, Clone, Copy)]
pub enum GameMode {

    /// Represents the `Survival` game mode for a player.
    /// See <https://minecraft.wiki/w/Game_mode#Survival>.
    Survival,

    /// Represents the `Creative` game mode for a player.
    /// See <https://minecraft.wiki/w/Game_mode#Creative>.
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
