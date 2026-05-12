use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackWool {
}


impl BlockState for BlackWool {
    fn to_id(self) -> i32 {
        return 2108;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2108 {
            return Some(BlackWool {
            });
        }
        return None;
    }
}

