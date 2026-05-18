//! Module for the [`Difficulty`] enum.

/// The [`Difficulty`] enum stores variants for the 4 Minecraft difficulties:
/// `Peaceful`, `Easy`, `Normal` and `Hard`.
///
/// See the variants' documentation or the Minecraft wiki for information on what the different
/// difficulties do.
/// <https://minecraft.wiki/w/Difficulty>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {

    /// **Peaceful** [`Difficulty`].
    /// See <https://minecraft.wiki/w/Difficulty#Peaceful> for more information about peaceful mode.
    Peaceful,

    /// **Easy** [`Difficulty`].
    /// See <https://minecraft.wiki/w/Difficulty#Easy> for more information about easy mode.
    Easy,

    /// **Normal** [`Difficulty`].
    /// See <https://minecraft.wiki/w/Difficulty#Normal> for more information about normal mode.
    Normal,

    Hard,
}

impl TryFrom<u8> for Difficulty {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err(()),
        };
    }
}

impl From<Difficulty> for u8 {
    fn from(value: Difficulty) -> Self {
        return match value {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        };
    }
}
