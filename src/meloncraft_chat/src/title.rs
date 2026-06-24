//! Types for title messages.

/// Determines where the title message should be displayed to the client.
/// See the fields for more details.
#[derive(Debug, Clone)]
pub enum TitlePosition {
    /// Display the message in the middle of the screen.
    /// Largest font size.
    Title,

    /// Display the message just below where a [`TitlePosition::Title`] would be,
    /// in a smaller font size.
    ///
    /// > IMPORTANT: You MUST send a title with [`TitlePosition::Title`] *as well* as a `Subtitle`.
    /// > If you do not also send a title, the client doesn't show the subtitle.
    Subtitle,

    /// Display the message just above the user's hotbar.
    /// Seemingly the smallest font size.
    Actionbar,
}

/// The amount of time that a title message takes to fade in, stay on the screen, and
/// fade out.
#[derive(Debug, Clone)]
pub struct TitleTimings {
    /// The number of ticks that it takes for any titles to fade *in* on the client's screen.
    pub fade_in_ticks: i32,

    /// The number of ticks that the title message should be fully shown for.
    pub stay_ticks: i32,

    /// The number of ticks that it takes for any titles to fade *out* on the client's screen.
    pub fade_out_ticks: i32,
}
