use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoneyBlock {
}


impl BlockState for HoneyBlock {
    fn to_id(self) -> i32 {
        return 21614;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21614 {
            return Some(HoneyBlock {
            });
        }
        return None;
    }
}

