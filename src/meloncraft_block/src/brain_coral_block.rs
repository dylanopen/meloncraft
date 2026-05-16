use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrainCoralBlock {
}


impl BlockState for BrainCoralBlock {
    fn to_id(&self) -> i32 {
        return 14941;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14941 {
            return Some(BrainCoralBlock {
            });
        }
        return None;
    }
}

