use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawGoldBlock {
}


impl BlockState for RawGoldBlock {
    fn to_id(self) -> i32 {
        return 29377;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29377 {
            return Some(RawGoldBlock {
            });
        }
        return None;
    }
}

