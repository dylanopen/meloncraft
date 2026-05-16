use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffBricks {
}


impl BlockState for TuffBricks {
    fn to_id(&self) -> i32 {
        return 24073;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24073 {
            return Some(TuffBricks {
            });
        }
        return None;
    }
}

