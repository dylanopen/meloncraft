use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkConcretePowder {
}


impl BlockState for PinkConcretePowder {
    fn to_id(self) -> i32 {
        return 14850;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14850 {
            return Some(PinkConcretePowder {
            });
        }
        return None;
    }
}

