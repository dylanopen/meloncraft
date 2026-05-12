use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledTuffBricks {
}


impl BlockState for ChiseledTuffBricks {
    fn to_id(self) -> i32 {
        return 24484;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24484 {
            return Some(ChiseledTuffBricks {
            });
        }
        return None;
    }
}

