use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedNetherBricks {
}


impl BlockState for RedNetherBricks {
    fn to_id(self) -> i32 {
        return 14645;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14645 {
            return Some(RedNetherBricks {
            });
        }
        return None;
    }
}

