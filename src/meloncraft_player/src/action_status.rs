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