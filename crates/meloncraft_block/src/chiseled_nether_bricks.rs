use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledNetherBricks {
}


impl BlockState for ChiseledNetherBricks {
    fn to_id(&self) -> i32 {
        return 22891;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22891 {
            return Some(ChiseledNetherBricks {
            });
        }
        return None;
    }
}

