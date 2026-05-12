use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledStoneBricks {
}


impl BlockState for ChiseledStoneBricks {
    fn to_id(self) -> i32 {
        return 7556;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7556 {
            return Some(ChiseledStoneBricks {
            });
        }
        return None;
    }
}

