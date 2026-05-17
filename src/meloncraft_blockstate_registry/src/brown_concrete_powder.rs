use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownConcretePowder {
}


impl BlockState for BrownConcretePowder {
    fn to_id(&self) -> i32 {
        return 14856;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14856 {
            return Some(BrownConcretePowder {
            });
        }
        return None;
    }
}

