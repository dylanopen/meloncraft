use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnowBlock {
}


impl BlockState for SnowBlock {
    fn to_id(&self) -> i32 {
        return 6727;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6727 {
            return Some(SnowBlock {
            });
        }
        return None;
    }
}

