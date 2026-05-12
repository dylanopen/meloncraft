use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanConcretePowder {
}


impl BlockState for CyanConcretePowder {
    fn to_id(self) -> i32 {
        return 14853;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14853 {
            return Some(CyanConcretePowder {
            });
        }
        return None;
    }
}

