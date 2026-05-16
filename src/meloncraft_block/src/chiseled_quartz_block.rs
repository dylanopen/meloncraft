use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledQuartzBlock {
}


impl BlockState for ChiseledQuartzBlock {
    fn to_id(&self) -> i32 {
        return 11122;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11122 {
            return Some(ChiseledQuartzBlock {
            });
        }
        return None;
    }
}

