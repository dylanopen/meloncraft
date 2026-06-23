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
