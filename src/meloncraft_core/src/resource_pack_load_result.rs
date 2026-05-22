//! Module for enum [`ResourcePackLoadResult`].

/// Variants representing the result of loading a resource pack.
/// This is usually sent serverbound after a client finishes (or fails) to download a resource pack
/// that the server requested them to load.
///
/// ## Integer representation
/// The variants for [`ResourcePackLoadResult`] can be represented as integers in packets. Their IDs
/// are specified in the individual variant documentation, and `From` and `TryFrom` are implemented
/// to convert between the enum and its integer representation.
///
/// See <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Resource_Pack_Response_(configuration)>.
#[derive(Debug, Clone)]
pub enum ResourcePackLoadResult {

    /// The client successfully loaded the resource pack.
    /// This is sent by the client after successfully downloading *and loading* the resource
    /// pack requested. It may not be the first result received.
    /// **ID `0` in packets.**
    Success,

    /// The client declined to load the resource pack. This is because they chose not to load the
    /// pack when prompted.
    /// **ID `1` in packets.**
    Declined,

    /// The client failed to load the resource pack. This is probably because an error occurred while
    /// downloading or loading the pack, such as a network error.
    /// **ID `2` in packets.**
    Failed,

    /// The client accepted to load the resource pack, but has not finished downloading it yet. This
    /// is probably sent by the client immediately after accepting to load the resource pack, and is
    /// followed by either the `Success` variant if the client finishes downloading and loading the
    /// pack, or the `Failed` variant if the client fails to download or load the pack.
    /// **ID `3` in packets.**
    Accepted,

    /// The client successfully downloaded the resource pack, but has not finished loading it yet.
    /// This should be sent after the download has finished, but their client hasn't finished
    /// actually deserializing and loading the pack yet. It should be followed by either the
    /// `Success` variant if the client finishes loading the pack, or the `Failed` variant if they
    /// fails to load the pack.
    Downloaded,

    /// The client couldn't download the resource pack because the URL provided by the server was
    /// invalid. This is probably sent close to immediately after the
    /// [`ResourcePackLoadResult::Accepted`] variant, since the client would only try to download
    /// the pack after accepting to load it.
    /// **ID `5` in packets.**
    InvalidUrl,

    /// **ID `6` in packets.**
    ReloadFailed,

    /// **ID `7` in packets.**
    Discarded,
}

impl TryFrom<i32> for ResourcePackLoadResult {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(ResourcePackLoadResult::Success),
            1 => Ok(ResourcePackLoadResult::Declined),
            2 => Ok(ResourcePackLoadResult::Failed),
            3 => Ok(ResourcePackLoadResult::Accepted),
            4 => Ok(ResourcePackLoadResult::Downloaded),
            5 => Ok(ResourcePackLoadResult::InvalidUrl),
            6 => Ok(ResourcePackLoadResult::ReloadFailed),
            7 => Ok(ResourcePackLoadResult::Discarded),
            _ => Err(()),
        };
    }
}

impl From<ResourcePackLoadResult> for i32 {
    fn from(value: ResourcePackLoadResult) -> Self {
        return match value {
            ResourcePackLoadResult::Success => 0,
            ResourcePackLoadResult::Declined => 1,
            ResourcePackLoadResult::Failed => 2,
            ResourcePackLoadResult::Accepted => 3,
            ResourcePackLoadResult::Downloaded => 4,
            ResourcePackLoadResult::InvalidUrl => 5,
            ResourcePackLoadResult::ReloadFailed => 6,
            ResourcePackLoadResult::Discarded => 7,
        };
    }
}

