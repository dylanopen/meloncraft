use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedCornflower {}

impl BlockState for PottedCornflower {
    fn to_id(&self) -> i32 {
        return 10450;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10450 {
            return Some(PottedCornflower {});
        }
        return None;
    }
}
