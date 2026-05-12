use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothStone {
}


impl BlockState for SmoothStone {
    fn to_id(self) -> i32 {
        return 13278;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13278 {
            return Some(SmoothStone {
            });
        }
        return None;
    }
}

