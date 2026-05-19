//! Module for [`DisconnectReport`] struct.

/// Stores a single disconnect report.
/// These are shown to the client if they get disconnected or, sometimes, if their game crashes.
///
/// As a server owner, you can create multiple [`DisconnectReport`]s and send them in the packet
/// listed below, to change the disconnect message a client receives.
///
/// ## Packets
/// Used in the `ClientboundCustomReportDetails` packet during *configuration*.
/// See that packet's documentation for more information.
#[derive(Debug, Clone)]
pub struct DisconnectReport {

    /// The (main) title of the disconnect report shown to the client.
    /// You should probably keep this as a short summary, and save longer explanations for the
    /// [`DisconnectReport::description`] field.
    ///
    /// ## Constraints
    /// - The [`DisconnectReport::title`] must be **less than or equal to** `128` characters in
    ///   length, or packets may fail to send.
    pub title: String,

    pub description: String,
}
