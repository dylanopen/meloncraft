#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerActionStatus {
    StartedDigging,
    CancelledDigging,
    FinishedDigging,
    DropItemStack,
    DropSingleItem,
    UpdateHeldItem,
    SwapOffhand,
}

impl TryFrom<u8> for PlayerActionStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(Self::StartedDigging),
            1 => Ok(Self::CancelledDigging),
            2 => Ok(Self::FinishedDigging),
            3 => Ok(Self::DropItemStack),
            4 => Ok(Self::DropSingleItem),
            5 => Ok(Self::UpdateHeldItem),
            6 => Ok(Self::SwapOffhand),
            _ => Err(()),
        }
    }
}

impl From<PlayerActionStatus> for u8 {
    fn from(value: PlayerActionStatus) -> Self {
        return match value {
            PlayerActionStatus::StartedDigging => 0,
            PlayerActionStatus::CancelledDigging => 1,
            PlayerActionStatus::FinishedDigging => 2,
            PlayerActionStatus::DropItemStack => 3,
            PlayerActionStatus::DropSingleItem => 4,
            PlayerActionStatus::UpdateHeldItem => 5,
            PlayerActionStatus::SwapOffhand => 6,
        }
    }
}

impl From<PlayerActionStatus> for i32 {
    fn from(value: PlayerActionStatus) -> Self {
        return u8::from(value).into();
    }
}

impl TryFrom<i32> for PlayerActionStatus {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return Self::try_from(u8::try_from(value).map_err(|_| ())?);
    }
}
