use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedConcretePowder {
}


impl BlockState for RedConcretePowder {
    fn to_id(self) -> i32 {
        return 14858;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14858 {
            return Some(RedConcretePowder {
            });
        }
        return None;
    }
}

