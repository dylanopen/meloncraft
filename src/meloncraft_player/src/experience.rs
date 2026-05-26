//! Module for [`Experience`] and related helper functions.

/// Get the total number of experience levels a player has, based on their experience *total* (see
/// [`Experience`].
///
/// See <https://minecraft.wiki/w/Experience#Leveling_up> for the formula explaining this.
#[must_use]
pub fn total_to_level(total: i32) -> f32 {
    #[expect(clippy::as_conversions, clippy::cast_precision_loss, reason = "Needed to convert i32 to f32")]
    let total = total as f32;
    if total < 352.0 {
        return (total + 9.0).sqrt() - 3.0;
    }
    if total < 1507.0 {
        return 81.0/10.0 + (0.4 * (total - 7839.0/40.0)).sqrt();
    }
    return 325.0/18.0 + (2.0/9.0 * (total - 54215.0/72.0)).sqrt();
}



