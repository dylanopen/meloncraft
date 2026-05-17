use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndStoneBricks {
}


impl BlockState for EndStoneBricks {
    fn to_id(&self) -> i32 {
        return 14594;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14594 {
            return Some(EndStoneBricks {
            });
        }
        return None;
    }
}

