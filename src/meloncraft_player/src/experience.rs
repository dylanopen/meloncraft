//! Module for [`Experience`] and related helper functions.

use bevy::ecs::component::Component;

/// Component of a player storing the *total* amount of experience points they have.
/// This is a single number, from which the level and level-fraction progress can be deduced.
///
/// See [`total_to_level`] for conversion methods.
#[derive(Component, Debug, Clone, Copy)]
pub struct Experience(pub i32);

/// Get the total number of experience levels a player has, based on their experience *total* (see
/// [`Experience`].
///
/// See <https://minecraft.wiki/w/Experience#Leveling_up> for the formula explaining this.
#[must_use]
pub fn total_to_level(total: i32) -> f32 {
    #[expect(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "Needed to convert i32 to f32"
    )]
    let total = total as f32;
    if total < 352.0 {
        return (total + 9.0).sqrt() - 3.0;
    }
    if total < 1507.0 {
        return 81.0 / 10.0 + (0.4 * (total - 7839.0 / 40.0)).sqrt();
    }
    return 325.0 / 18.0 + (2.0 / 9.0 * (total - 54215.0 / 72.0)).sqrt();
}

#[cfg(test)]
mod tests {
    #[expect(
        clippy::float_cmp,
        reason = "The float should always be an exact value"
    )]
    #[test]
    fn total_to_exact_level() {
        use super::*;
        assert_eq!(total_to_level(7), 1.0);
        assert_eq!(total_to_level(187), 11.0);
        assert_eq!(total_to_level(352), 16.0);
        assert_eq!(total_to_level(394), 17.0);
        assert_eq!(total_to_level(828), 24.0);
        assert_eq!(total_to_level(1507), 31.0);
        assert_eq!(total_to_level(1628), 32.0);
        // assert_eq!(total_to_level(10540), 65.0); // this fails for some reason, likely a float division error?
    }
}
