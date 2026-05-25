//! Module for the [`Difficulty`] enum.

use bevy::ecs::resource::Resource;

/// The [`Difficulty`] enum stores variants for the 4 Minecraft difficulties:
/// `Peaceful`, `Easy`, `Normal` and `Hard`.
///
/// See the variants' documentation or the Minecraft wiki for information on what the different
/// difficulties do, as well as their protocol IDs.
/// <https://minecraft.wiki/w/Difficulty>.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {

    /// **Peaceful** [`Difficulty`].
    /// Protocol ID: `0`.
    /// See <https://minecraft.wiki/w/Difficulty#Peaceful> for more information about peaceful mode.
    Peaceful,

    /// **Easy** [`Difficulty`].
    /// Protocol ID: `1`.
    /// See <https://minecraft.wiki/w/Difficulty#Easy> for more information about easy mode.
    Easy,

    /// **Normal** [`Difficulty`].
    /// Protocol ID: `2`.
    /// See <https://minecraft.wiki/w/Difficulty#Normal> for more information about normal mode.
    Normal,

    /// **Hard** [`Difficulty`].
    /// Protocol ID: `3`.
    /// See <https://minecraft.wiki/w/Difficulty#Hard> for more information about hard mode.
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
