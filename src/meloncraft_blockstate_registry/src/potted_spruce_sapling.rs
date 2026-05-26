use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedSpruceSapling {}

impl BlockState for PottedSpruceSapling {
    fn to_id(&self) -> i32 {
        return 10431;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10431 {
            return Some(PottedSpruceSapling {});
        }
        return None;
    }
}
