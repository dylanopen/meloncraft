use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BubbleCoralBlock {}

impl BlockState for BubbleCoralBlock {
    fn to_id(&self) -> i32 {
        return 14942;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14942 {
            return Some(BubbleCoralBlock {});
        }
        return None;
    }
}
