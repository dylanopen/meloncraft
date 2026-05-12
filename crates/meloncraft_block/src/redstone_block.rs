use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneBlock {
}


impl BlockState for RedstoneBlock {
    fn to_id(self) -> i32 {
        return 11109;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11109 {
            return Some(RedstoneBlock {
            });
        }
        return None;
    }
}

