#[derive(Debug, Clone)]
#[repr(i32)]
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
        match value {
            0 => Ok(ResourcePackLoadResult::Success),
            1 => Ok(ResourcePackLoadResult::Declined),
            2 => Ok(ResourcePackLoadResult::Failed),
            3 => Ok(ResourcePackLoadResult::Accepted),
            4 => Ok(ResourcePackLoadResult::Downloaded),
            5 => Ok(ResourcePackLoadResult::InvalidUrl),
            6 => Ok(ResourcePackLoadResult::ReloadFailed),
            7 => Ok(ResourcePackLoadResult::Discarded),
            _ => Err(()),
        }
    }
}
