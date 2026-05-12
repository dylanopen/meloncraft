use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedWool {
}


impl BlockState for RedWool {
    fn to_id(self) -> i32 {
        return 2107;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2107 {
            return Some(RedWool {
            });
        }
        return None;
    }
}

