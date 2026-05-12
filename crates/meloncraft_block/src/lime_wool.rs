use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeWool {
}


impl BlockState for LimeWool {
    fn to_id(self) -> i32 {
        return 2098;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2098 {
            return Some(LimeWool {
            });
        }
        return None;
    }
}

