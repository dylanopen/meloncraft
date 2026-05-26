use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedCactus {}

impl BlockState for PottedCactus {
    fn to_id(&self) -> i32 {
        return 10456;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10456 {
            return Some(PottedCactus {});
        }
        return None;
    }
}
