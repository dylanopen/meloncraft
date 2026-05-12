use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleConcretePowder {
}


impl BlockState for PurpleConcretePowder {
    fn to_id(self) -> i32 {
        return 14854;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14854 {
            return Some(PurpleConcretePowder {
            });
        }
        return None;
    }
}

