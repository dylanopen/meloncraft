//! Module for enum [`ResourcePackLoadResult`].

/// Variants representing the result of loading a resource pack.
/// This is usually sent serverbound after a client finishes (or fails) to download a resource pack
/// that the server requested them to load.
///
/// ## Integer representation
/// The variants for [`ResourcePackLoadResult`] can be represented as integers in packets. Their IDs
/// are specified in the individual variant documentation, and `From` and `TryFrom` are implemented
/// to convert between the enum and its integer representation.
#[derive(Debug, Clone)]
pub enum ResourcePackLoadResult {

    Success = 0,

    Declined = 1,

    Failed = 2,

    Accepted = 3,

    Downloaded = 4,

    InvalidUrl = 5,

    ReloadFailed = 6,

    Discarded = 7,
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

