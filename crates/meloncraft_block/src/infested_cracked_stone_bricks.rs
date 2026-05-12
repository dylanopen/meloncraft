use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfestedCrackedStoneBricks {
}


impl BlockState for InfestedCrackedStoneBricks {
    fn to_id(self) -> i32 {
        return 7563;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7563 {
            return Some(InfestedCrackedStoneBricks {
            });
        }
        return None;
    }
}

