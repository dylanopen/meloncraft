use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBubbleCoralBlock {
}


impl BlockState for DeadBubbleCoralBlock {
    fn to_id(&self) -> i32 {
        return 14937;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14937 {
            return Some(DeadBubbleCoralBlock {
            });
        }
        return None;
    }
}

